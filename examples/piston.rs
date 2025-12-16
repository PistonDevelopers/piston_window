#[cfg(not(feature = "batteries"))]
fn main() {
    eprintln!("ERROR:\nCount not start the Piston game engine.\n\nTip: Enable the `batteries` feature.");
}

#[cfg(feature = "batteries")]
fn main() -> Result<(), ()> {
    use piston_window::{piston_script::run, dyon::Module};

    let file = std::env::args_os().nth(1)
        .and_then(|s| s.into_string().ok());
    let file = if let Some(file) = file {
        use std::env::set_current_dir;
        use std::path::PathBuf;

        let path: PathBuf = (&file).into();
        if let Some(parent) = path.parent() {
            if let Err(_) = set_current_dir(parent) {
                file
            } else {
                path.file_name().unwrap().to_str().unwrap().to_owned()
            }
        } else {
            file
        }
    } else {
        println!("piston <file.dyon>");
        return Err(());
    };

    run(&file, Module::new(), |f| f())
}
