use std::io::{self, Write};

use crate::models::{Category, SubCategory};

pub mod index;

pub fn menu(out: &mut dyn Write) -> io::Result<()> {
    let mut profession = Vec::new();
    profession.push(SubCategory { path: "craft", name: "Fabricant de runes" });
    profession.push(SubCategory { path: "test 2", name: "Nécromant" });
    profession.push(SubCategory { path: "test 2", name: "Alchémiste" });
    profession.push(SubCategory { path: "test 2", name: "Tailleur de pierre" });
    profession.push(SubCategory { path: "test 2", name: "Ebeniste" });
    profession.push(SubCategory { path: "test 2", name: "Forgeron" });
    profession.push(SubCategory { path: "test 2", name: "Maroquinier" });
    profession.push(SubCategory { path: "test 2", name: "Joaillier" });

    let mut menu = Vec::new();
    menu.push(Category {
        name: "🛠 Métier",
        sub_categories: profession,
    });

    crate::templates::menu(
        out,
        &menu,
    )
}