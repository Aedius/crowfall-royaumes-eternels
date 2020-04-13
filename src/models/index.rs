use crate::models::Article;

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
