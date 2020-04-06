use std::env;
use std::io::{self, Write, Error as IoError, Cursor};


use tiny_http::{Method, Request, Response, Server, StatusCode};

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
use templates::index;
use templates::statics::StaticFile;
use templates::statics::favicon_ico;

//slide
use templates::statics::map_jpg;
use templates::statics::elfettes_png;
use templates::statics::village_png;


use crate::models::{Category, Config, Index, Slide, Article};


fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    let mut categories = Vec::new();
    categories.push(Category { path: "test", name: "TEST" });
    categories.push(Category { path: "test 2", name: "TEST 2" });

    let config = Config {
        categories
    };

    println!("listening on 8000");

    for request in server.incoming_requests() {
        println!("received request!\n, method: {:?}\n, url: {:?}\n, headers: {:?}\n",
                 request.method(),
                 request.url(),
                 request.headers(),
        );
        if request.method() == &Method::Get {
            match handle_get(request, &config) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e)
                }
            }
        } else {
            match request.respond(Response::new_empty(StatusCode(405))) {
                Ok(_) => {}
                Err(e) => {
                    println!("{}", e)
                }
            }
        }
    }
}

fn handle_get(request: Request, config: &Config) -> Result<(), IoError> {
    let url = request.url();

    if url == "/" {
        let response = handle_index(&config);
        return request.respond(response);
    }

    if url == "/favicon.ico" {
        let mime_type = favicon_ico.mime.to_string();
        let mut response = tiny_http::Response::from_data(favicon_ico.content);
        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], mime_type.into_bytes()).unwrap();
        response.add_header(header);
        return request.respond(response);
    }

    let tokens: Vec<&str> = url.split("/").collect();

    if tokens.len() != 3 {
        return request.respond(Response::new_empty(StatusCode(404)));
    }

    if let Some(data) = StaticFile::get(tokens[2]) {
        let mime_type = data.mime.to_string();

        let mut response = tiny_http::Response::from_data(data.content);
        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], mime_type.into_bytes()).unwrap();
        response.add_header(header);
        return request.respond(response);
    }

    request.respond(Response::new_empty(StatusCode(404)))
}

fn handle_index(config: &Config) -> Response<Cursor<Vec<u8>>> {
    let data = Index {
        slides: vec![Slide {
            src: map_jpg.name,
            alt: "Map",
            url: None,
            content: "Plusieurs cartes",
        }, Slide {
            src: elfettes_png.name,
            alt: "elfettes",
            url: None,
            content: "oui",
        }, Slide {
            src: village_png.name,
            alt: "village",
            url: None,
            content: "non",
        },
        ],
        articles: vec![Article {
            title: "D"
        }],
    };

    let mut response = tiny_http::Response::from_string(r2s(|o| index(o, &config, &data)));
    let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
    response.add_header(header);
    response
}

fn r2s<Call>(call: Call) -> String
    where
        Call: FnOnce(&mut dyn Write) -> io::Result<()>,
{
    let mut buf = Vec::new();
    call(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}


mod models {
    pub struct Config<'a> {
        pub categories: Vec<Category<'a>>
    }

    pub struct Category<'a> {
        pub path: &'a str,
        pub name: &'a str,
    }

    pub struct Index<'a> {
        pub slides: Vec<Slide<'a>>,
        pub articles: Vec<Article<'a>>,
    }

    pub struct Slide<'a> {
        pub src: &'a str,
        pub alt: &'a str,
        pub url: Option<&'a str>,
        pub content: &'a str,
    }

    pub struct Article<'a> {
        pub title: &'a str,
    }
}