extern crate http;
extern crate iron;
extern crate router;

use std::os::getenv;
use std::io::net::ip::{Ipv4Addr, Port};
use iron::{Iron, Request, Response, IronResult, Set};
use iron::response::modifiers::{Status, Body};
use iron::status;
use router::{Router, Params};

fn get_server_port() -> Port {
    getenv("PORT")
    .and_then(|s| from_str::<Port>(s.as_slice()))
    .unwrap_or(8080)
}

// Serves a customized string to the user.  Try accessing "/lunch".
fn hello_name(req: &mut Request) -> IronResult<Response> {
    let params = req.extensions.find::<Router,Params>().unwrap();
    let name = params.find("name").unwrap();
    let resp = Response::new()
    .set(Status(status::Ok))
    .set(Body(format!("Yo dawg, {}!", name)));
    Ok(resp)
}

fn main() {
    let mut router = Router::new();
    router.get("/:name", hello_name);
    Iron::new(router).listen(Ipv4Addr(0, 0, 0, 0), get_server_port());
}
