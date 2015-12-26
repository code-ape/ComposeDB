mod server;

extern crate iron;
extern crate router;
extern crate rustc_serialize;

use server::server::run_server;

fn main() {
    run_server();
}
