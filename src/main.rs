use std::fs::File;
use std::path::Path;

use tiny_http::{Method, Response, Server, StatusCode};

mod craft;

fn main() {
    let server = Server::http("0.0.0.0:8000").unwrap();

    println!("listening on 8000");

    for request in server.incoming_requests() {
        println!("received request!\n, method: {:?}\n, url: {:?}\n, headers: {:?}\n",
                 request.method(),
                 request.url(),
                 request.headers(),
        );
        // if request.method() == &Method::Get {
        //
        //     let response = tiny_http::Response::from_file(File::open(&Path::new("ionic/public/assets/fond.jpg")).unwrap());
        //
        //     let _ = request.respond(response);
        //
        // } else {
        //     match request.respond(Response::new_empty(StatusCode(405))) {
        //         Ok(_) => {}
        //         Err(e) => {
        //             println!("{}", e)
        //         }
        //     }
        // }
    }
}

