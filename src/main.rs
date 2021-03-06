extern crate futures;
extern crate hyper;
extern crate rand;
extern crate htmlescape;

use futures::future::Future;

use rand::Rng;

struct CookieService<'a> {
    fortunes: &'a Vec<String>
}

impl<'a> hyper::server::Service for CookieService<'a> {
    type Request = hyper::server::Request;
    type Response = hyper::server::Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response,Error=Self::Error>>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let fortune = rand::thread_rng().choose(self.fortunes).unwrap().to_owned();
        if req.uri().path() == "/html" {
            let resp = format!("<!DOCTYPE html>
            <html>
            <head>
            <meta property=\"og:title\" content=\"Forune\" />
            <meta property=\"og:type\" content=\"website\" />
            <meta property=\"og:description\" content=\"{}\" />
            </head>
            <body>
            <pre>{}</pre>
            </body>
            </html>", htmlescape::encode_minimal(&fortune), fortune);
            Box::new(futures::future::ok(Self::Response::new()
                                         .with_header(hyper::header::ContentLength(resp.len() as u64))
                                         .with_header(hyper::header::ContentType::html())
                                         .with_body(resp)))
        }
        else {
            let resp = fortune;
            Box::new(futures::future::ok(Self::Response::new()
                                         .with_header(hyper::header::ContentLength(resp.len() as u64))
                                         .with_header(hyper::header::ContentType::plaintext())
                                         .with_body(resp)))
        }
    }
}

fn main() {
    let addr = std::net::SocketAddr::new(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(0,0,0,0)),
        std::env::var("PORT").unwrap_or("7000".to_owned())
        .parse().unwrap()
        );
    let fortunes = include_str!("../cookie").split("\n%\n").map(|s|s.to_owned()).collect::<Vec<_>>();
    let server = hyper::server::Http::new()
        .bind(&addr, move || Ok(CookieService {
            fortunes: &fortunes
        }))
        .unwrap();
    server.run().unwrap();
}
