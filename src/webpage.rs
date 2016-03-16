use std::fs::{File};
use std::io::{Read};

use hyper;
use hyper::server::{Request, Response, Server};
use hyper::status::StatusCode;

const HTTP_ADDR: &'static str = "0.0.0.0:1980";
const HTML_DATA: &'static str = "html/index.html";

// Returns val from Ok(val) or sets the response to return an InternalServerError.
macro_rules! try_or_server_err {
    ($expr:expr, $res:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            *($res).status_mut() = StatusCode::InternalServerError;
            return;
        }
    })
}

fn req_handler(req: Request, mut res: Response) {
    match req.method {
        hyper::Get => {
            // Read the file HTML_DATA into buf.
            let mut buf = String::new();
            let mut page =  try_or_server_err!(File::open(HTML_DATA), res);
            try_or_server_err!(page.read_to_string(&mut buf), res);

            // And return buf as the response.
            *res.status_mut() = StatusCode::Ok;
            res.send(&buf.as_bytes()).unwrap();
        },
        _ => *res.status_mut() = StatusCode::BadRequest,
    }
}

pub fn serve() {
    println!("HTTP: listening on {}.", HTTP_ADDR);
    match Server::http(HTTP_ADDR) {
        Ok(server) => match server.handle(req_handler) {
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        },
        Err(e) => println!("{:?}", e),
    }
}
