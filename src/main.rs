extern crate hyper;
extern crate rustc_serialize;
extern crate websocket;

mod webpage;
mod chess_server;
mod board;

fn main() {
    chess_server::start();
    webpage::serve();
}
