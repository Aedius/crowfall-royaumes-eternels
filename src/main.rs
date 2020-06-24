use tiny_http::{Method, Request, Response, Server, StatusCode};

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
        if request.method() == &Method::Get {
            match request.respond(Response::new_empty(StatusCode(200))) {
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

