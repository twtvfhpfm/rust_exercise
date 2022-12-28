mod dlna;

fn main() {
    println!("Hello, world!");

    let mut my_dlna = dlna::DLNA::new("F7CA5454-3F48-4390-8009-36ade4ee6e9e", 5876);
    my_dlna.start_tcp_server();
    my_dlna.start_broadcast();
    my_dlna.join();
}
