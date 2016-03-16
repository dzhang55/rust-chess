extern crate hyper;
extern crate rustc_serialize;
extern crate websocket;

mod webpage;
mod chatserver;

fn main() {
    chatserver::start();
    webpage::serve();
}
