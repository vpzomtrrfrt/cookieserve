extern crate futures;
extern crate hyper;

use futures::future::Future;

struct CookieService;

impl hyper::server::Service for CookieService {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response,Error=Self::Error>>;

    fn call(&self, _req: Self::Request) -> Self::Future {
        let resp = "test";
        Box::new(futures::future::ok(Self::Response::new()
                                     .with_header(hyper::header::ContentLength(resp.len() as u64))
                                     .with_body(resp)))
    }
}

fn main() {
    let addr = std::net::SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(0,0,0,0)),
        std::env::var("PORT").unwrap_or("7000".to_owned())
        .parse().unwrap()
        );
    let server = hyper::server::Http::new()
        .bind(&addr, || Ok(CookieService))
        .unwrap();
    server.run().unwrap();
}
