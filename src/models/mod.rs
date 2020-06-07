pub mod index;

pub const URL_TOOLS_COOKING: &str = "/tools/craft/cooking";

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