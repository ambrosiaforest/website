use glob::glob;
use pulldown_cmark::{Options, Parser, html};
use std::fs;
use std::fs::File;
use std::io::{self, Error};
use std::path::{Path, PathBuf};

fn create_md(filename: &str) -> Result<(), Error> {
    let input_dir = PathBuf::from("src");
    let output_dir = PathBuf::from("dst");

    std::fs::create_dir_all(&output_dir)?;

    let header = fs::read_to_string(input_dir.join("_header.xhtml"))?;
    let footer = fs::read_to_string(input_dir.join("_footer.xhtml"))?;
    let markdown = fs::read_to_string(input_dir.join(format!("{}.md", filename)))?;

    let options = Options::empty();

    let parser = Parser::new_ext(&markdown, options);

    let mut body = String::new();
    html::push_html(&mut body, parser);

    let output_path = output_dir.join(format!("{}.xhtml", filename));
    fs::write(&output_path, format!("{header}{body}{footer}"))?;

    Ok(())
}

fn create_file(filename: &str, extension: &str) -> Result<(), Error> {
    let input_dir = PathBuf::from("src");
    let output_dir = PathBuf::from("dst");

    let output_path = output_dir.join(format!("{}.{}", filename, extension));

    std::fs::create_dir_all(output_dir)?;

    let mut output = File::create(&output_path)?;

    let file = format!("{}.{}", filename, extension);
    let input_path = input_dir.join(file);
    let mut input = File::open(&input_path)?;
    io::copy(&mut input, &mut output)?;

    Ok(())
}

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
        "rs" => Ok(()),
        _ => create_file(filename, extension),
    };
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
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
