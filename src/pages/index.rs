use std::io::Cursor;

use tiny_http::Response;

use crate::models::Article;
use crate::models::index::{Band, Index, Slide};
use crate::templates::index;
use crate::templates::statics::elfettes_png;
//slide
use crate::templates::statics::map_jpg;
use crate::templates::statics::village_png;

pub fn handle() -> Response<Cursor<Vec<u8>>> {
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
            img: village_png.name,
            title: "premier article",
            url: "url",
            author: "Aedius",
            published: "2 avril 2020",
            updated: "4 avril 2020",
            resume: "TEST TEST TEST",
        }],
    };

    let mut response = tiny_http::Response::from_string(crate::r2s(|o| index(o, &data)));
    let header = tiny_http::Header::from_bytes(&b"Content-Type"[..], &b"text/html; charset=utf-8"[..]).unwrap();
    response.add_header(header);
    response
}