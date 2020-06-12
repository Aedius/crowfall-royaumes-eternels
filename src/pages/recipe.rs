use std::collections::HashMap;
use std::io::Cursor;

use tiny_http::Response;
use tiny_http::StatusCode;

use crate::craft::cooking::get_all_recipe;
use crate::craft::Item::Crafted;
use crate::craft::Recipe;
use crate::templates::recipe;

pub fn handle(recipeName: &str) -> Response<Cursor<Vec<u8>>> {
    let recipes = get_all_recipe();

    let mut recipeMap = HashMap::new();

    let mut currentRecipe = None;

    for recipe in recipes {
        let out = recipe.output.0.clone();
        if out.get_information().is_some() {
            let info = out.get_information().unwrap();
            if recipeName == info.key {
                currentRecipe = Some(recipe.clone())
            }
        }
        recipeMap.insert(out, recipe.clone());
    }

    println!("{:?}", currentRecipe);

    if currentRecipe.is_none() {
        let mut response = tiny_http::Response::from_string("Not found");
        let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
        response.add_header(header);
        return response;
    }


    let mut response = tiny_http::Response::from_string(crate::r2s(|o| recipe(o, currentRecipe.unwrap())));
    let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
    response.add_header(header);
    response
}