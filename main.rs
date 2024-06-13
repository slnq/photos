use std::io::{self, Write};
use std::path::Path;

fn copy_file(source_file: &str, destination_dir: &str) -> io::Result<()> {
    let source_path = Path::new(source_file);
    let destination_path = Path::new(destination_dir).join(source_path.file_name().unwrap());

    std::fs::copy(&source_path, &destination_path)?;
    Ok(())
}

fn copy_images(source_dir: &str, destination_dir: &str) -> io::Result<()> {
    let source_path = Path::new(source_dir);
    let destination_path = Path::new(destination_dir).join("imgs");

    if !destination_path.exists() {
        std::fs::create_dir_all(&destination_path)?;
    }

    let mut entries: Vec<_> = std::fs::read_dir(source_path)?.collect();
    entries.sort_by_key(|entry| entry.as_ref().unwrap().path());
    entries.reverse();

    for entry in entries {
        let entry = entry?;
        let source_file = entry.path();

        if let Some(filename) = source_file.file_name() {
            let destination_file = destination_path.join(filename);
            std::fs::copy(&source_file, &destination_file)?;
        }
    }

    Ok(())
}

fn generate_html(filename: &str) -> String {
    format!(
        r#"<html><head><link rel="stylesheet" type="text/css" href="main.css">
        <meta property="og:title" content="📷" />
        <meta property="og:description" content="📸" />
        <meta property="og:type" content="website" />
        <meta property="og:url" content="https://slnq.github.io/photos/{}.html" />
        <meta property="og:image" content="https://slnq.github.io/photos/imgs/{}" />
        <meta property="og:site_name" content="📸" />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:title" content="📷" />
        <meta name="twitter:description" content="📸" />
        <meta name="twitter:image" content="https://slnq.github.io/photos/imgs/{}" />
        <link href="./imgs/f.ico" rel="icon">
        </head><body class="bdy"><img src="./imgs/{}" class="blur1"/><img src="./imgs/{}" class="photo1"/></body></html>"#,
        filename, filename, filename, filename, filename
    )
}

fn generate_index_html(image_files: &[String]) -> String {
    let mut html = String::from("<html><head><link rel=\"stylesheet\" type=\"text/css\" href=\"main.css\"><link href=\"./imgs/f.ico\" rel=\"icon\"></head><body><div class=\"container\">");
    for filename in image_files {
        html.push_str(&format!(
            r#"<a href="./{}.html"><img src="./imgs/{}" class="blur"/><img src="./imgs/{}" class="photo"/></a>"#,
            filename.split('.').next().unwrap(), filename, filename
        ));
    }
    html.push_str("</div></body></html>");
    html
}

fn create_html_files(img_dir: &str, output_dir: &str) -> io::Result<Vec<String>> {
    let mut image_files = Vec::new();

    let mut entries: Vec<_> = std::fs::read_dir(img_dir)?.collect();
    entries.sort_by_key(|entry| entry.as_ref().unwrap().path());
    entries.reverse();

    for entry in entries {
        let entry = entry?;
        if let Some(filename) = entry.file_name().to_str() {
            // Exclude main.css file
            if filename == "main.css" || filename == "f.ico" {
                continue;
            }

            image_files.push(filename.to_string());

            let html = generate_html(filename);
            let output_path = Path::new(output_dir).join(format!("{}.html", filename.split('.').next().unwrap()));
            let mut file = std::fs::File::create(output_path)?;
            file.write_all(html.as_bytes())?;
        }
    }

    Ok(image_files)
}

fn generate_and_write_index_html(image_files: &[String], output_dir: &str) -> io::Result<()> {
    let index_html = generate_index_html(image_files);
    let index_path = Path::new(output_dir).join("index.html");
    let mut index_file = std::fs::File::create(index_path)?;
    index_file.write_all(index_html.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let img_dir = "imgs";
    let output_dir = "public";

    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;

    // Copy images from imgs directory to public/imgs directory
    copy_images(img_dir, output_dir)?;

    // Copy main.css from imgs directory to public directory
    copy_file("imgs/main.css", output_dir)?;

    // Create HTML files for each image
    let image_files = create_html_files(img_dir, output_dir)?;

    // Generate and write index.html
    generate_and_write_index_html(&image_files, output_dir)?;

    Ok(())
}
