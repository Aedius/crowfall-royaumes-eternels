#[macro_use]
extern crate lazy_static;

use std::env;
use std::io::{self, Error as IoError, Write};

use tiny_http::{Method, Request, Response, Server, StatusCode};

use templates::statics::favicon_ico;
//slide
use templates::statics::StaticFile;

use crate::models::{URL_CRAFT_RECIPE_RE, URL_TOOLS_COOKING};
use crate::pages::cooking::handle as handle_cooking;
use crate::pages::index::handle as handle_index;
use crate::pages::recipe::handle as handle_recipe;

mod models;
mod pages;
mod craft;


include!(concat!(env!("OUT_DIR"), "/templates.rs"));
fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();


    println!("listening on 8000");

    for request in server.incoming_requests() {
        println!("received request!\n, method: {:?}\n, url: {:?}\n, headers: {:?}\n",
                 request.method(),
                 request.url(),
                 request.headers(),
        );
        if request.method() == &Method::Get {
            match handle_get(request) {
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

fn handle_get(request: Request) -> Result<(), IoError> {
    let url = request.url();

    if url == "/" {
        let response = handle_index();
        return request.respond(response);
    }

    if url == URL_TOOLS_COOKING {
        let response = handle_cooking();
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

    if tokens.len() == 3 && tokens[1] == "static" {
        if let Some(data) = StaticFile::get(tokens[2]) {
            let mime_type = data.mime.to_string();

            let mut response = tiny_http::Response::from_data(data.content);
            let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], mime_type.into_bytes()).unwrap();
            response.add_header(header);
            return request.respond(response);
        }
    }

    if URL_CRAFT_RECIPE_RE.is_match(url) {
        let caps = URL_CRAFT_RECIPE_RE.captures(url).unwrap();

        let as_text = caps.get(1).map_or("", |m| m.as_str());

        let response = handle_recipe(as_text);

        return request.respond(response);
    }

    request.respond(Response::new_empty(StatusCode(404)))
}


fn r2s<Call>(call: Call) -> String
    where
        Call: FnOnce(&mut dyn Write) -> io::Result<()>,
{
    let mut buf = Vec::new();
    call(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}

