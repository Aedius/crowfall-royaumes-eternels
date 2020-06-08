use std::io::Cursor;

use tiny_http::Response;

use crate::craft::cooking::get_all_recipe;
use crate::templates::craft;

pub fn handle() -> Response<Cursor<Vec<u8>>> {
    let recipes = get_all_recipe();

    let mut response = tiny_http::Response::from_string(crate::r2s(|o| craft(o, &recipes)));
    let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
    response.add_header(header);
    response
}