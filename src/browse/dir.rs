use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;
use urlencoding::encode;
use core::fmt;

static VIDEO_PAGE_TEMPLATE: &'static str = include_str!("video.html");

#[derive(Debug)]
pub enum BrowseError {
    IoError(std::io::Error)
}

impl fmt::Display for BrowseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BrowseError::IoError(ref e) => e.fmt(f),
        }
    }
}

impl From<std::io::Error> for BrowseError {
    fn from(err: std::io::Error) -> BrowseError {
        BrowseError::IoError(err)
    }
}

// from errpr

pub fn video_html(encoded_file_path: String) -> String {
    return VIDEO_PAGE_TEMPLATE.replace("{file_name}", &encoded_file_path);
}

pub fn list_files_to_html(root_dir: &str, sub_dir_path: &str) -> Result<String, BrowseError> {
    let mut result: String = String::new();
    result.push_str("<html><body><ul>");
    let mut path = PathBuf::from(root_dir);
    path.push(sub_dir_path);

    let dir = fs::read_dir(path)?;
    for entry in dir {
        match entry {
            Ok(dir_entry) => {
                let entry = entry_to_html(dir_entry, sub_dir_path)?;
                result.push_str(entry.as_str());
            }
            Err(err) => eprintln!("Error: {}", err)
        }
    }
    result.push_str("</ul></body></html>");
    return Ok(result);
}

fn entry_to_html(dir_entry: DirEntry, sub_dir_path: &str) -> Result<String, BrowseError> {
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