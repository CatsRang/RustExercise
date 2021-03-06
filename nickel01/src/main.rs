#[macro_use] extern crate nickel;

use nickel::{Nickel, HttpRouter, Mountable, Request, Response, MiddlewareResult};

fn main() {
    let mut server = Nickel::new();

    server.mount("/some/", handler_01);

    server.utilize(explicit_router());
    server.utilize(explicit_router02());

    server.listen("127.0.0.1:6767").unwrap();
}

fn handler_01 <'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    println!("> handler, requested url = {}", req.origin.uri);
    res.next_middleware()
}

fn explicit_router() -> nickel::Router {
    let mut router = Nickel::router();
    router.get("/some/**", handler_01);
    router
}

fn explicit_router02() -> nickel::Router {
    let mut router = Nickel::router();

    router.get("/some/**", middleware! {
        | req, resp |
        println!("> reques 02 url: {}", req.origin.uri);
        "router 02"
    });

    router
}

