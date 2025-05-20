use server::Server;

mod server;
mod http;

fn main() {
    let address:String=String::from("127.0.0.1:8080");
    let srv=Server::new(address);
    srv.run();
}
