use std::env;
use std::io::{self, Write, Error as IoError};


use tiny_http::{Method, Request, Response, Server, StatusCode};

include!(concat!(env!("OUT_DIR"), "/templates.rs"));
use templates::index;
use templates::statics::StaticFile;
use templates::statics::favicon_ico;
use crate::models::Category;

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    let mut categories = Vec::new();
    categories.push(Category{ path: "test", name: "TEST" });



    println!("listening on 8000");

    for request in server.incoming_requests() {
        println!("received request!\n, method: {:?}\n, url: {:?}\n, headers: {:?}\n",
                 request.method(),
                 request.url(),
                 request.headers(),
        );
        if request.method() == &Method::Get {
            match handle_get(request, &categories) {
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

fn handle_get(request: Request, categories : &Vec<Category>) -> Result<(), IoError> {
    let url = request.url();

    if url == "/" {
        let mut response = tiny_http::Response::from_string(r2s(|o| index(o, categories)));
        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
        response.add_header(header);
        return request.respond(response);
    }

    if url == "/favicon.ico" {

        let mime_type = favicon_ico.mime.to_string();
        let mut response = tiny_http::Response::from_data(favicon_ico.content);
        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], mime_type.into_bytes()).unwrap();
        response.add_header(header);
        return request.respond(response);
    }

    let tokens:Vec<&str>= url.split("/").collect();

    println!("{:?}", tokens);

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

fn r2s<Call>(call: Call) -> String
    where
        Call: FnOnce(&mut dyn Write) -> io::Result<()>,
{
    let mut buf = Vec::new();
    call(&mut buf).unwrap();
    String::from_utf8(buf).unwrap()
}


mod models {

    pub struct Category<'a> {
        pub path: &'a str,
        pub name: &'a str,
    }

}