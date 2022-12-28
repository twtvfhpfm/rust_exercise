use std::{thread, net::{UdpSocket, Ipv4Addr, TcpListener, TcpStream}, io::ErrorKind, time::{Duration, Instant}, fs, process::abort, u32::MIN};
use std::io::{Read, Write};
use http_client::HTTPClient;
use std::sync::{Arc, Mutex};

use self::tcp_server::ThreadPool;

mod tcp_server;
mod http_client;

pub struct DLNA {
    bcast_thread: Option<thread::JoinHandle<()>>,
    uuid: String,
    port: u16,
    local_ip: String,
    http_client: Arc<Mutex<HTTPClient>>
}

fn getip() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}

impl DLNA {
    pub fn new(uuid: &str, port: u16) -> DLNA {
        DLNA {
            bcast_thread: None,
            uuid: String::from(uuid),
            port,
            local_ip: getip().unwrap(),
            http_client: Arc::new(Mutex::new(HTTPClient::new())),
        }
    }

    pub fn start_broadcast(&mut self) {
        let socket = UdpSocket::bind("0.0.0.0:1900").expect("bind failed");
        socket.join_multicast_v4(&Ipv4Addr::new(239, 255, 255, 250), &Ipv4Addr::UNSPECIFIED).expect("join multicase failed");
        socket.set_nonblocking(true).expect("set nonblocking failed");

        let uuid = self.uuid.clone();
        let location_vec = vec![
            format!("http://{}:{}/description.xml", self.local_ip, self.port), 
        ];
        let nt_vec = vec![
            "upnp:rootdevice",
            "urn:schemas-upnp-org:device:MediaRenderer:1",
            "urn:schemas-upnp-org:device:RenderingControl:1",
            "urn:schemas-upnp-org:device:ConnectionManager:1",
        ];

        let thread = thread::spawn(move || {
            let mut buf = [0u8; 4096];
            let mut last_bcast_time = Instant::now();
            loop {
                let notify_pat = concat!("NOTIFY * HTTP/1.1\r\n",
                                        "HOST: 239.255.255.250:1900\r\n",
                                        "CACHE-CONTROL: max-age=60\r\n",
                                        "LOCATION: <location>\r\n",
                                        "NT: <nt>\r\n",
                                        "NTS: ssdp:alive\r\n",
                                        "SERVER: Linux/4.9.113, UPnP/1.0, Portable SDK for UPnP devices/1.6.13\r\n",
                                        "USN: uuid:<uuid>::<nt>\r\n\r\n");

                match socket.recv_from(&mut buf) {
                    Ok((num_bytes, from_addr)) => {
                        //println!("received {} bytes from {}: \n{}", num_bytes, from_addr, String::from_utf8_lossy(&buf[..num_bytes]));
                    }
                    Err(ref err) if err.kind() != ErrorKind::WouldBlock => {
                        println!("recv error: {}", err);
                    }
                    _ => {
                        thread::sleep(Duration::from_millis(100));
                    }
                }

                let now = Instant::now();
                if now.duration_since(last_bcast_time).as_millis() > 1000 {
                    last_bcast_time = now;

                    for nt in &nt_vec {
                        let send_data = notify_pat.replace("<location>", &location_vec[0])
                            .replace("<uuid>", &uuid)
                            .replace("<nt>", &nt);

                        let bytes = send_data.as_bytes();

                        match socket.send_to(bytes, "239.255.255.250:1900") {
                            Ok(num_bytes)  => {
                                if num_bytes == bytes.len() {
                                    //println!("send {} bytes successfully", num_bytes);
                                } else {
                                    println!("send bytes {} < {}", num_bytes, bytes.len());
                                }
                            }
                            Err(err) => {
                                println!("send error: {}", err);
                            }

                        }
                    }
                }
            }

        });

        self.bcast_thread = Some(thread);
    }

    pub fn join(&mut self) {
        if let Some(thread) = self.bcast_thread.take() {
            thread.join().unwrap();
        }
    }

    pub fn start_tcp_server(&self) {
        let addr = format!("0.0.0.0:{}", self.port);
        let listener = TcpListener::bind(&addr).unwrap();
        let pool = ThreadPool::new(4);
        let http_client = Arc::clone(&self.http_client);
        thread::spawn(move || {
            for stream in listener.incoming() {
                let stream = stream.unwrap();
                let http_client = Arc::clone(&http_client);
                pool.execute(||{
                    DLNA::handle_connection(stream, http_client);
                });
            }

        });
    }

    fn parse_data(data: &str) -> Option<usize> {
        let mut bytes = 0;
        if let Some(n) = data.find("\r\n\r\n") {
            bytes = n;
        } else {
            println!("no header end");
            return None;
        }

        for line in data[..bytes].lines() {
            if line.to_lowercase().contains("content-length") {
                let len: usize = line[15..].trim().parse().unwrap();
                //println!("content-length: {}", len);
                let need_len = bytes + len + 4;
                if data.len() < need_len {
                    println!("need {} bytes", need_len);
                    return None;
                }

                return Some(need_len);
            }
        }

        //println!("no content-length");
        return Some(data.len());
    }

    fn get_uri(request: &str) -> Option<String> {
        if let Some(begin) = request.find("<CurrentURI>") {
            if let Some(end) = request.find("</CurrentURI>") {
                let mut uri = String::from(&request[begin+12..end]);
                while uri.contains("&amp;") {
                    uri = uri.replace("&amp;", "&");
                }
                Some(uri)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn handle_connection(mut stream: TcpStream, http_client: Arc<Mutex<HTTPClient>>)
    {
        let mut buffer = [0u8; 4096];
        let mut bytes = 0;

        loop{
            match stream.read(&mut buffer[bytes..]) {
                Ok(n) => {
                    if n==0 {
                        //println!("recv finish");
                        return;
                    }
                    bytes += n;
                    //println!("read {} bytes", bytes);
                }
                Err(_) => {
                    println!("recv error");
                    return
                }
            }

            let mut req_len = 0;
            if let Some(len) = DLNA::parse_data(&String::from_utf8_lossy(&buffer[..bytes])) {
                req_len = len;
                //println!("req_len: {}", req_len);
            } else{
                continue;
            }

            let request = String::from_utf8_lossy(&buffer[..req_len]);
            //println!("------------------------------");
            //println!("------------------------------\nRequest: {}", request);

            let get_description = "GET /description.xml HTTP/1.1\r\n";
            let get_avtransport_desc = "GET /upnp/AVTransport/desc HTTP/1.1\r\n";
            let get_connectionmanager_desc = "GET /upnp/RenderingControl/desc HTTP/1.1\r\n";
            let get_renderingcontrol_desc = "GET /upnp/ConnectionManager/desc HTTP/1.1\r\n";
            let post_action = "POST /upnp/AVTransport/action HTTP/1.1\r\n";

            let (debug, file_name) = if request.starts_with(get_description){
                (false, "description.xml")
            } else if request.starts_with(get_avtransport_desc) {
                (false, "avtransport_desc.xml")
            } else if request.starts_with(get_renderingcontrol_desc) {
                (false, "renderingcontrol_desc.xml")
            } else if request.starts_with(get_connectionmanager_desc) {
                (false, "connectionmanager_desc.xml")
            } else if request.starts_with(post_action) {
                if request.contains("#Stop") {
                    (true, "stop.xml")
                } else if request.contains("#GetTransportInfo") {
                    (true, "gettransportinfo.xml")
                } else if request.contains("#SetAVTransportURI") {
                    if let Some(uri) = DLNA::get_uri(&request){
                        println!("find uri: {}", uri);
                        http_client.lock().unwrap().download(&uri, "output.flv");
                    }
                    (true, "setavtransporturi.xml")
                } else {
                    (true, "none")
                }
            }
            else{
                (true, "none")
            };

            let left = bytes - req_len;
            if left > 0 {
                //move data
                buffer.copy_within(req_len..bytes, 0);
            }
            bytes = 0;

            let content = fs::read_to_string(file_name).unwrap_or(String::from(""));
            let response = format!("HTTP/1.1 200 OK\r\nContent-Type: text/xml\r\nSERVER: Linux/4.9.113, UPnP/1.0, Portable SDK for UPnP devices/1.6.13\r\nContent-Length: {}\r\n\r\n{}", content.len(), content);
            if debug {
                //println!("Response: {}", &response);
            }
            match stream.write(response.as_bytes()) {
                Ok(n) => {
                    if n == 0 {
                        println!("write 0 bytes");
                        return;
                    } else {
                        stream.flush().unwrap();
                    }
                },
                Err(_) => {
                    println!("write failed");
                    return;
                }
            }
        }
    }
}
