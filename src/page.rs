//extern crate iron;
//extern crate static_file;
//extern crate mount;

// This example serves the docs from target/doc/static_file at /doc/
//
// Run `cargo doc && cargo test && ./target/doc_server`, then
// point your browser to http://127.0.0.1:3000/doc/

use std::path::Path;

use iron::Iron;
use staticfile::Static;
use mount::Mount;

pub fn main() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("html/")));
    //mount.mount("/css", Static::new(Path::new("html/css")));
    //mount.mount("/js", Static::new(Path::new("html/js")));
    //mount.mount("/img", Static::new(Path::new("html/img")));

    //Iron::new(mount).listen(Ipv4Addr::new(127, 0, 0, 1), 1980);
    Iron::new(mount).http("localhost:1980").unwrap();

    println!("Doc server running on http://localhost:1980/");
}