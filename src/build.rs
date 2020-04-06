use ructe::{Result, Ructe};

fn main() -> Result<()> {

    println!("cargo:rerun-if-changed=scss");
    println!("cargo:rerun-if-changed=scss/partials");
    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=templates/slide");

    let mut ructe = Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("static")?;
    statics.add_files("static/slide")?;
    statics.add_sass_file("scss/base.scss")?;

    ructe.compile_templates("templates")
}