use std::io::{self, Write};

use crate::models::{Category, SubCategory, URL_TOOLS_COOKING};

pub mod index;
pub mod cooking;


pub fn menu(out: &mut dyn Write) -> io::Result<()> {
    let mut profession = Vec::new();
    profession.push(SubCategory { path: URL_TOOLS_COOKING, name: "cuisine" });

    let mut menu = Vec::new();
    menu.push(Category {
        name: "🛠 Craft",
        sub_categories: profession,
    });

    crate::templates::menu(
        out,
        &menu,
    )
}