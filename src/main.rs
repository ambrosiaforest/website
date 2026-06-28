use glob::glob;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Read, Write, Error};

fn create_xhtml(filename: &str) -> Result<(), Error> {
    let input_dir = PathBuf::from("src");
    let output_dir = PathBuf::from("dst");

    let header = "_header.xhtml";
    let footer = "_footer.xhtml";
    let output_path = output_dir.join(format!("{}.xhtml", filename));

    std::fs::create_dir_all(output_dir)?;

    let mut output = File::create(&output_path)?;

    for file in &[header, format!("{}.xhtml", filename).as_str(), footer] {
        let input_path = input_dir.join(file);
        let mut input = File::open(&input_path)?;
        io::copy(&mut input, &mut output)?;
    }

    Ok(())
}

fn sort_file(filename: &str, extension: &str) {
    match extension {
        "xhtml" => create_xhtml(filename),
        "md" => create_md(filename),
        "rs" => (),
        _ => create_file(filename)
    }
}

fn split_file(path: &Path) -> Option<(&str, &str)> {
    let filename = path.file_stem()?.to_str()?;
    let extension = path.extension()?.to_str()?;

    Some((filename, extension))
}

fn main() {
    for entry in glob("src/*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Some((filename, extension)) = split_file(&path) {
                    sort_file(filename, extension);
                }
            },
            Err(e) => println!("{:?}", e),
        }
    }
}
