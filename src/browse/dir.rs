use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use urlencoding::encode;

static VIDEO_PAGE_TEMPLATE: &'static str = include_str!("video.html");

pub fn video_html(encoded_file_path: String) -> String {
    return VIDEO_PAGE_TEMPLATE.replace("{file_name}", &encoded_file_path);
}


pub fn list_files_to_html(root_dir: &str, sub_dir_path: &str) -> String {
    let mut result: String = String::new();
    result.push_str("<html><body><ul>");
    let mut path = PathBuf::from(root_dir);
    path.push(sub_dir_path);

    let dir = fs::read_dir(path);
    match dir {
        Result::Err(_) => { result.push_str(format!("ERROR Reading dir: {}", root_dir).as_str()) }
        Result::Ok(read_dir) => {
            for entry in read_dir {
                match entry {
                    Ok(dir_entry) => {
                        match entry_to_html(dir_entry, sub_dir_path) {
                            Ok(entry) => {
                                result.push_str(entry.as_str());
                            }
                            Err(_) => {}
                        }
                    }
                    Err(err) => eprintln!("Error: {}", err)
                }
            }
        }
    }
    result.push_str("</ul></body></html>");
    return result;
}

pub fn entry_to_html(dir_entry: DirEntry, sub_dir_path: &str) -> Result<String, std::io::Error> {
    let file = dir_entry.file_name();
    let meta = dir_entry.metadata()?;
    return match file.to_str() {
        Some(name) => {
            let path;
            if sub_dir_path.is_empty() {
                path = encode(name);
            } else {
                path = encode(format!("{}/{}", sub_dir_path, name).as_str())
            }
            if meta.is_file() {
                return Ok(format!("<li><a href=\"/video/{}\">{}</a></li>", path, name));
            }
            if meta.is_dir() {
                return Ok(format!("<li><a href=\"/dir/{}\">[ {} ]</a></li>", path, name));
            }
            Ok(String::new())
        }
        None => { Ok(String::new()) }
    };
}