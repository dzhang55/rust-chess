extern crate hyper;
extern crate iron;
extern crate staticfile;
extern crate mount;
extern crate rustc_serialize;
extern crate websocket;

mod webpage;
mod page;
mod chess_server;
mod board;

fn main() {
    chess_server::start();
    //webpage::serve();
    page::main();
}
