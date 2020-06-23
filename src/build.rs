use ructe::{Result, Ructe};

fn main() -> Result<()> {

    println!("cargo:rerun-if-changed=static");
    println!("cargo:rerun-if-changed=static/slide");
    println!("cargo:rerun-if-changed=ionic/build/assets");
    println!("cargo:rerun-if-changed=ionic/build/static");

    let mut ructe = Ructe::from_env()?;
    let mut statics = ructe.statics()?;
    statics.add_files("static")?;
    statics.add_files("static/slide")?;
    statics.add_files("ionic/build/assets")?;
    statics.add_files("ionic/build/static")?;

    ructe.compile_templates("templates")
}