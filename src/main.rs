use std::env;
use std::io::{self, Cursor, Error as IoError, Write};

use tiny_http::{Method, Request, Response, Server, StatusCode};

use templates::index;
use templates::statics::elfettes_png;
use templates::statics::favicon_ico;
//slide
use templates::statics::map_jpg;
use templates::statics::StaticFile;
use templates::statics::village_png;

use crate::models::{Article, Band, Category, Config, Index, Slide, SubCategory};

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    let mut profession = Vec::new();
    profession.push(SubCategory { path: "craft", name: "Fabricant de runes" });
    profession.push(SubCategory { path: "test 2", name: "Nécromant" });
    profession.push(SubCategory { path: "test 2", name: "Alchémiste" });
    profession.push(SubCategory { path: "test 2", name: "Tailleur de pierre" });
    profession.push(SubCategory { path: "test 2", name: "Ebeniste" });
    profession.push(SubCategory { path: "test 2", name: "Forgeron" });
    profession.push(SubCategory { path: "test 2", name: "Maroquinier" });
    profession.push(SubCategory { path: "test 2", name: "Joaillier" });

    let mut menu = Vec::new();
    menu.push(Category {
        name: "🛠 Métier",
        sub_categories: profession,
    });

    let config = Config {
        categories: menu
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
            src: village_png.name,
            alt: "village",
            url: None,
            content: "Bienvenue au village",
        }, Slide {
            src: elfettes_png.name,
            alt: "elfettes",
            url: None,
            content: "Les elfes vous attendent",
        }, Slide {
            src: map_jpg.name,
            alt: "Map",
            url: None,
            content: "Sur une des campagnes",
        }, ],
        band: Some(Band {
            url: "https://www.youtube.com/channel/UCit0NjPf6lMXAwx2xnnOy2g",
            content: "Suivez le projet Maeve sur youtube !",
        }),
        articles: vec![Article {
            img: "img",
            title: "premier article",
            url: "url",
            author: "Aedius",
            published: "2 avril 2020",
            updated: "4 avril 2020",
            resume: "TEST TEST TEST",
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
        pub name: &'a str,
        pub sub_categories: Vec<SubCategory<'a>>,
    }

    pub struct SubCategory<'a> {
        pub path: &'a str,
        pub name: &'a str,
    }

    pub struct Index<'a> {
        pub slides: Vec<Slide<'a>>,
        pub articles: Vec<Article<'a>>,
        pub band: Option<Band<'a>>,
    }

    pub struct Slide<'a> {
        pub src: &'a str,
        pub alt: &'a str,
        pub url: Option<&'a str>,
        pub content: &'a str,
    }

    pub struct Band<'a> {
        pub url: &'a str,
        pub content: &'a str,
    }

    pub struct Article<'a> {
        pub img: &'a str,
        pub title: &'a str,
        pub url: &'a str,
        pub author: &'a str,
        pub published: &'a str,
        pub updated: &'a str,
        pub resume: &'a str,

    }
}