extern crate iron;
extern crate router;
extern crate rustc_serialize;

use std::io::Read;

use iron::prelude::*;
use iron::status;
use router::Router;
use rustc_serialize::json;

#[derive(RustcDecodable, RustcEncodable)]
struct Greeting {
    msg: String
}

fn main() {
    let mut router = Router::new();

    router.get("/", hello_world);
    router.post("/set", set_greeting);

    fn hello_world(_: &mut Request) -> IronResult<Response> {
        println!("Request for route '/'");
        let greeting = Greeting { msg: "Hello, World".to_string() };
        let payload = json::encode(&greeting).unwrap();
        println!("Responded with: {}", payload);
        Ok(Response::with((status::Ok, payload)))
    }

    // Receive a message by POST and play it back.
    fn set_greeting(req: &mut Request) -> IronResult<Response> {
        println!("Request for route '/set'");
        let mut payload = String::new();
        req.body.read_to_string(&mut payload).unwrap();
        let request: Greeting = json::decode(&payload).unwrap();
        let greeting = Greeting { msg: request.msg };
        let payload = json::encode(&greeting).unwrap();
        println!("Responded with: {}", payload);
        Ok(Response::with((status::Ok, payload)))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
