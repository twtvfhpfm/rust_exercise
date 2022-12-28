use std::{thread::{self, JoinHandle}, net::TcpStream, io::{Read, Write}, fs::File, time::Instant, sync::mpsc::{Sender, Receiver}};
use dns_lookup::lookup_host;
use std::time;
use std::sync::mpsc;

pub struct HTTPClient {
    url: Option<String>,
    file_name: Option<String>,
    thrd_hdl: Option<JoinHandle<()>>,
    sender: Option<Sender<String>>
}

struct Url {
    scheme: String,
    domain: String,
    port: u16,
    path: String,
}

impl HTTPClient {
    pub fn new() -> HTTPClient {
        HTTPClient {
            url: None,
            file_name: None,
            thrd_hdl: None,
            sender: None,
        }
    }

    pub fn download(&mut self, url: &str, file_name: &str) {
        if self.url.as_ref().unwrap_or(&String::from("")) == &url {
            println!("this url is already downloading");
            return;
        }

        if let Some(hdl) = &self.thrd_hdl {
            println!("send quit");
            self.sender.take().unwrap().send(String::from("quit"));
            println!("wait download thread to quit");
            self.thrd_hdl.take().unwrap().join();
        }

        let url_str = String::from(url);
        let file_name_str = String::from(file_name);
        let (tx, rx) = mpsc::channel::<String>();
        let hdl = thread::spawn(move || {
            HTTPClient::do_download(&url_str, &file_name_str, rx);
        });

        self.thrd_hdl = Some(hdl);
        self.url = Some(String::from(url));
        self.file_name = Some(String::from(file_name));
        self.sender = Some(tx);

    }

    fn parse_url(url: &str) -> Url {
        let mut scheme = String::from("http");
        let mut domain_idx = "http://".len();
        let mut port = 80;
        if url.starts_with("https://"){
            scheme = String::from("https");
            domain_idx += 1;
            port = 443;
        }

        let mut domain_end_idx = url.len();
        if let Some(domain_end) = url[domain_idx..].find("/") {
            domain_end_idx = domain_idx + domain_end;
        }

        let mut domain = &url[domain_idx..domain_end_idx];
        if let Some(idx) = domain.find(":") {
            port = domain[idx+1..].parse().unwrap();
            domain = &domain[..idx];
        }

        let path = &url[domain_end_idx..];

        Url{
            scheme,
            domain: String::from(domain),
            port,
            path: String::from(path),
        }
    }

    fn do_download(url: &str, file_name: &str, rx: Receiver<String>) {
        println!("parsing url");
        let url_info = HTTPClient::parse_url(&url);

        println!("look up host");
        let ips: Vec<std::net::IpAddr> = lookup_host(&url_info.domain).unwrap();
        println!("connecting to {}:{}", ips[0], url_info.port);
        if let Ok(mut stream) = TcpStream::connect((ips[0], url_info.port)){
            println!("connect success");
            let mut buffer = [0u8; 4096];
            let mut bytes = 0;

            let mut file = File::create(file_name).expect("create failed");
            let mut start_time = Instant::now();

            let send_data = format!("GET {} HTTP/1.1\r\nUser-Agent: Chrome (linux)\r\nAccept: */*\r\nAccept-Encoding: identity\r\nConnection: Keep-Alive\r\nHost:{}\r\n\r\n", url_info.path, url_info.domain);
            println!("send request: {}", send_data);
            stream.write(send_data.as_bytes()).expect("write failed");
            let mut find_body = false;

            loop{
                if let Ok(s) = rx.try_recv() {
                    match s.as_str() {
                        "quit" => {
                            println!("quit downloading");
                            return;
                        },
                        _ => {}
                    }
                }

                match stream.read(&mut buffer) {
                    Ok(n) => {
                        if n==0 {
                            println!("recv finish");
                            break;
                        }
                        bytes += n;

                        let mut start = 0;
                        if !find_body {
                            for i in 0..n-3 {
                                if buffer[i] == 0x0d
                                && buffer[i+1] == 0x0a
                                && buffer[i+2] == 0x0d
                                && buffer[i+3] == 0x0a{
                                    start = i+4;
                                    find_body = true;
                                    println!("recv response:\n{}", String::from_utf8_lossy(&buffer[..start]));
                                }
                            }
                        }
                        file.write(&buffer[start..n]).expect("write failed");
                        //println!("read {} bytes", bytes);
                    }
                    Err(_) => {
                        println!("recv error");
                        break;
                    }
                }
                if start_time.elapsed().as_millis() > 1000 {
                    println!("{} downloading.... {} B/s", find_body, bytes);
                    start_time = Instant::now();
                    bytes = 0;
                }
            }
            file.flush().expect("flush failed");
        }
        else {
            println!("tcp connect to {}:{} failed", url_info.domain, url_info.port);
        }
    }
}
