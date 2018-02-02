extern crate argparse;
extern crate futures;
extern crate hyper;
extern crate tokio_core;

use argparse::{ArgumentParser, Store, StoreTrue, List};
use futures::{Future, Stream};
use hyper::Client;
use tokio_core::reactor::Core;
use hyper::{Method, Request};
use hyper::header::{self, Headers, Raw};
use hyper::header::Header;

fn main() {
    struct Options {
        method: String,
        url: String,
        headers: Vec<String>,
        body: String,
        show_headers: bool,
        show_body: bool,
        show_status: bool
    }

    let mut options = Options {
        method: String::new(),
        url: String::new(),
        headers: Vec::new(),
        body: String::new(),
        show_headers: false,
        show_body: false,
        show_status: false,
    };

    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut options.method).add_argument(&"method", Store, "Http methods: GET, POST, PUT, DELETE").required();
        ap.refer(&mut options.url).add_argument(&"url", Store, "Url").required();
        ap.refer(&mut options.headers).add_option(&["-h"], List, "Request headers");
        ap.refer(&mut options.body).add_option(&["-b"], Store, "Request body");
        ap.refer(&mut options.show_headers).add_option(&["-p", "--show_headers"], StoreTrue, "Show response headers");
        ap.refer(&mut options.show_body).add_option(&["-z", "--show_body"], StoreTrue, "Show response body");
        ap.refer(&mut options.show_status).add_option(&["-r", "--show_status"], StoreTrue, "Show response status");
        ap.parse_args_or_exit();
    }

    match Core::new() {
        Ok(mut core) => {
            match options.url.parse() {
                Ok(uri) => {
                    let mut client = Client::new(&core.handle());
                    let method = if options.method == "post" {
                        Method::Post
                    } else if options.method == "put" {
                        Method::Put
                    } else if options.method == "delete" {
                        Method::Delete
                    } else {
                        Method::Get
                    };
                    let mut request = Request::new(method, uri);

                    options.headers.iter().map(|s| s.split(":").collect::<Vec<&str>>()).for_each(|s| s.iter().for_each(|v| println!("{:?}", v)));

                    let work = client.request(request).and_then(|res| {
                        if options.show_status {
                            println!("Response status: {}", res.status())
                        }
                        if options.show_headers {
                            println!("Response headers");
                            res.headers().iter().for_each(|header| println!("{}", header));
                        }
                        if options.show_body {
                            let mut body = String::new();
                            let _ = res.body().concat2().map_err(|_err| ()).map(|chunk| {
                                body = String::from_utf8_lossy(&chunk.to_vec()).to_string()
                            });
                            println!("Response body: {}", body);
                        }

                        futures::future::ok(())
                    });
                    let _ = core.run(work);
                }
                Err(_) => println!("errrr")
            }
        }
        Err(_) => println!("error")
    }
}

