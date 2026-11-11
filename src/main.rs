use glob::glob;
use pulldown_cmark::{Options, Parser, html};
use std::fs;
use std::io::Error;
use std::path::{Path, PathBuf};

fn split_post(stem: &str) -> Option<((u32, u32, u32), &str)> {
    let mut parts = stem.splitn(4, '-');
    let y = parts.next()?.parse().ok()?;
    let m = parts.next()?.parse().ok()?;
    let d = parts.next()?.parse().ok()?;
    let title = parts.next()?;
    Some(((y, m, d), title))
}

fn insert_blog() -> Result<(), Error> {
    let input_dir = PathBuf::from("www");
    let index_path = PathBuf::from("dst").join("index.xhtml");

    let mut posts = Vec::new();
    for entry in fs::read_dir(&input_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }

        let Some(stem) = path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if let Some((date, title)) = split_post(stem) {
            posts.push((date, stem.to_string(), title.replace('-', " ")));
        }
    }

    posts.sort_by(|a, b| b.0.cmp(&a.0));

    let mut list = String::new();
    for ((y, m, d), stem, title) in &posts {
        list.push_str(&format!(
            "<li>{y}-{m:02}-{d:02} <a href=\"{stem}.xhtml\">{title}</a></li>\n"
        ));
    }

    let index = fs::read_to_string(&index_path)?;
    fs::write(&index_path, index.replace("<!--blogs-->", &list))?;

    Ok(())
}

fn create_md(filename: &str) -> Result<(), Error> {
    let input_dir = PathBuf::from("www");
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
    let input_dir = PathBuf::from("www");
    let output_dir = PathBuf::from("dst");

    std::fs::create_dir_all(&output_dir)?;

    let input_path = input_dir.join(format!("{filename}.{extension}"));
    let output_path = output_dir.join(format!("{filename}.{extension}"));

    fs::copy(input_path, output_path)?;

    Ok(())
}

fn create_xhtml(filename: &str) -> Result<(), Error> {
    let input_dir = PathBuf::from("www");
    let output_dir = PathBuf::from("dst");

    std::fs::create_dir_all(&output_dir)?;

    let header = fs::read_to_string(input_dir.join("_header.xhtml"))?;
    let footer = fs::read_to_string(input_dir.join("_footer.xhtml"))?;
    let body = fs::read_to_string(input_dir.join(format!("{filename}.xhtml")))?;

    let output_path = output_dir.join(format!("{}.xhtml", filename));

    fs::write(&output_path, format!("{header}{body}{footer}"))?;

    Ok(())
}

fn sort_file(filename: &str, extension: &str) -> Result<(), Error> {
    match extension {
        "xhtml" => create_xhtml(filename),
        "md" => create_md(filename),
        _ => create_file(filename, extension),
    }
}

fn split_file(path: &Path) -> Option<(&str, &str)> {
    let filename = path.file_stem()?.to_str()?;
    let extension = path.extension()?.to_str()?;

    Some((filename, extension))
}

fn main() {
    for entry in glob("www/*").expect("Failed to read glob pattern") {
        match entry {
            Ok(path) => {
                if let Some((filename, extension)) = split_file(&path) {
                    if let Err(e) = sort_file(filename, extension) {
                        eprintln!("Failde to process {filename}.{extension}: {e}");
                    };
                }
            }
            Err(e) => println!("{:?}", e),
        }
    }

    if let Err(e) = insert_blog() {
        eprintln!("insert_blog falied: {e}");
    }
}
