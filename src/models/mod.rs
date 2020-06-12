use regex::Regex;

pub mod index;

pub const URL_TOOLS_COOKING: &str = "/tools/craft/cooking";

lazy_static! {
    pub static ref URL_CRAFT_RECIPE_RE: Regex = Regex::new(r"/tools/craft/recipe/([^/]+)$").unwrap();
}

pub struct Category<'a> {
    pub name: &'a str,
    pub sub_categories: Vec<SubCategory<'a>>,
}

pub struct SubCategory<'a> {
    pub path: &'a str,
    pub name: &'a str,
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