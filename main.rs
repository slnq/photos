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
        r#"<html><head>
		<meta charset="UTF-8">
		<link rel="stylesheet" type="text/css" href="each.css">
        <meta property="og:title" content="📷" />
        <meta property="og:description" content="📸" />
        <meta property="og:type" content="website" />
        <meta property="og:url" content="https://slnq.github.io/photos/{}.html" />
        <meta property="og:image" content="https://slnq.github.io/photos/imgs/{}" />
        <meta property="og:site_name" content="📸" />
        <meta name="twitter:card" content="summary_large_image" />
        <meta name="twitter:title" content="📷" /><meta name="twitter:description" content="📸" /><meta name="twitter:image" content="https://slnq.github.io/photos/imgs/{}" /><link href="./imgs/f.ico" rel="icon"></head><body class="bdy"><img src="./imgs/{}" class="photo1"/></body></html><div id="caption"></div><script src="https://cdn.rawgit.com/exif-js/exif-js/master/exif.js"></script><script>	window.onload = function() {{	  var element = document.getElementsByClassName('photo1')[0];	  var caption = document.getElementById('caption');	  if (element) {{		EXIF.getData(element, function() {{		  var userComment = EXIF.getTag(this, 'UserComment');		  if (userComment) {{			var trimmedComment = userComment.slice(8);			var decodedComment = decodeUserComment(trimmedComment);			caption.innerHTML += decodedComment;		  }}		}});	  }} else {{		console.error('Class "photo1" not found');	  }}	}};	function decodeUserComment(comment) {{	  try {{		var decoder = new TextDecoder('utf-16le');		return decoder.decode(new Uint8Array(comment));	  }} catch (e) {{		console.error('Failed to decode UserComment', e);		return comment;	  }}	}}  </script>
		"#,
        filename.split('.').next().unwrap(), filename, filename, filename
    )
}

fn generate_index_html(image_files: &[String], a1: u32, a2: u32) -> String {
    let mut html = String::from("<html><head><link rel=\"stylesheet\" type=\"text/css\" href=\"main.css\"><link href=\"./imgs/f.ico\" rel=\"icon\"></head><body><div class=\"container\">");
    let mut i = 0;
	for filename in image_files {
		if i < a1-1 {
			html.push_str(&format!(
				r#"<img src="./imgs/{}" class="d1"/>"#,
				 filename
			));
		} else if i == a1-1 {
			html.push_str(&format!(
				r#"<img src="./imgs/{}" class="d1" id="lst"/>"#,
				 filename
			));
		}
		else if i == a1 + a2-1 ||  i == (image_files.len()-1).try_into().unwrap() {
			html.push_str(&format!(
				r#"<img src="./imgs/{}" class="photo" id="lst"/>"#,
				 filename
			));
			} else {
		html.push_str(&format!(
			r#"<img src="./imgs/{}" class="photo"/>"#,
             filename
        ));
		}
		i+=1;
		if i == a1 {
			html.push_str("</br>");
		} else if i == a1 + a2 {
			html.push_str("</br>");
		} 
    }
    html.push_str("</div><script src=\"https://cdn.jsdelivr.net/npm/exif-js\"></script><div id=\"lightbox\">    <img id=\"lightboxImg\">    <div id=\"caption\"></div></div><script>window.onload = function() {    var images = document.querySelectorAll('.container img');    images.forEach(function(img) {        img.addEventListener('click', function() {            openLightbox(img);         });    });    var lightbox = document.getElementById('lightbox');    var lightboxImg = document.getElementById('lightboxImg');    lightbox.addEventListener('click', function(e) {        if (e.target !== lightboxImg) {            closeLightbox();         }    });    lightboxImg.addEventListener('click', function() {        var src = lightboxImg.src;        var filename = src.split('/').pop();         var basename = filename.split('.').slice(0, -1).join('.');         window.location.href = basename + \".html\";     });};function openLightbox(element) {    var lightbox = document.getElementById('lightbox');    var lightboxImg = document.getElementById('lightboxImg');    var caption = document.getElementById('caption');    lightboxImg.src = element.src;      caption.innerHTML = element.alt;      EXIF.getData(element, function() {        var userComment = EXIF.getTag(this, 'UserComment');        if (userComment) {            var trimmedComment = userComment.slice(8);             var decodedComment = decodeUserComment(trimmedComment);            caption.innerHTML += decodedComment;        }    });    lightbox.style.display = \"flex\";  lightbox.style.flexDirection = \"column\";  }function closeLightbox() {    var lightbox = document.getElementById('lightbox');    lightbox.style.display = \"none\";  }function decodeUserComment(userComment) {    var uint8Array = new Uint8Array(userComment);    var decoder = new TextDecoder('utf-16le');    return decoder.decode(uint8Array);}</script></body></html>");
    html
}

fn create_html_files(img_dir: &str, output_dir: &str, a1: &mut u32, a2: &mut u32) -> io::Result<Vec<String>> {
    let mut image_files = Vec::new();

    let mut entries: Vec<_> = std::fs::read_dir(img_dir)?.collect();
    entries.sort_by_key(|entry| entry.as_ref().unwrap().path());
    entries.reverse();
	
	let mut que: Vec<Vec<_>> = vec![Vec::new(), Vec::new(), Vec::new()];
	let mut que_w: Vec<u32> = vec![0, 0, 0];
	let mut i = 0;
	let margin = 15;
	let height = 400;

    for entry in entries {
		let entry = entry?;
        if let Some(filename) = entry.file_name().to_str() {
			if filename == "main.css" ||filename == "each.css"|| filename == "f.ico" {
				continue;
            }
			
			i+=1;
			let ent_i = image::io::Reader::open(entry.path())?.into_dimensions().unwrap();

			if (1..=3).contains(&i) {
				que[i - 1].push(filename.to_string());
				que_w[i - 1] += ent_i.0 * height / ent_i.1 + margin;
			}
			else if que_w[0] < que_w[1] {
				if que_w[0] < que_w[2] {
					que_w[0] += ent_i.0 * height / ent_i.1 + margin;
					que[0].push(filename.to_string());
					*a1 += 1;
				} else {
					que_w[2] += ent_i.0 * height / ent_i.1 + margin;
					que[2].push(filename.to_string());
				}
			} else {
					if que_w[1] < que_w[2] {
						que_w[1] += ent_i.0 * height / ent_i.1 + margin;
						que[1].push(filename.to_string());
						*a2 += 1;
					} else {
						que_w[2] += ent_i.0 * height / ent_i.1 + margin;
						que[2].push(filename.to_string());
					}
			}
			
			//image_files.push(filename.to_string());
            let html = generate_html(filename);
            let output_path = Path::new(output_dir).join(format!("{}.html", filename.split('.').next().unwrap()));
            let mut file = std::fs::File::create(output_path)?;
            file.write_all(html.as_bytes())?;
        }
    }

	for que in que.iter() {
		for filename in que {
			image_files.push(filename.to_string());
		}
	}

    Ok(image_files)
}

fn generate_and_write_index_html(image_files: &[String], output_dir: &str, a1: u32, a2: u32) -> io::Result<()> {
    let index_html = generate_index_html(image_files, a1, a2);
    let index_path = Path::new(output_dir).join("index.html");
    let mut index_file = std::fs::File::create(index_path)?;
    index_file.write_all(index_html.as_bytes())?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    let img_dir = "imgs";
    let output_dir = "public";

	let mut a1 = 1;
	let mut a2 = 1;

    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;

    // Copy images from imgs directory to public/imgs directory
    copy_images(img_dir, output_dir)?;

    // Copy main.css from imgs directory to public directory
    copy_file("imgs/main.css", output_dir)?;
    copy_file("imgs/each.css", output_dir)?;

    // Create HTML files for each image
    let image_files = create_html_files(img_dir, output_dir, &mut a1, &mut a2)?;

    // Generate and write index.html
    generate_and_write_index_html(&image_files, output_dir, a1, a2)?;

    Ok(())
}
