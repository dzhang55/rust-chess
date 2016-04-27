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

/// Serves the html page.
pub fn main() {
    let mut mount = Mount::new();
    mount.mount("/", Static::new(Path::new("html/")));
    Iron::new(mount).http("localhost:1980").unwrap();
    println!("Doc server running on http://localhost:1980/");
}