use std::fs;
use std::path::{Path, PathBuf};

pub fn sanitize_path(request_path: &str, base_dir: &Path) -> Option<PathBuf> {
    let relative = request_path.trim_start_matches('/');
    let joined = base_dir.join(relative);

    match joined.canonicalize() {
        Ok(full_path) => {
            let base = match base_dir.canonicalize().ok() {
                Some(path) => path,
                None => {
                    println!("Failed to canonicalize base_dir");
                    return None;
                }
            };

            if full_path.starts_with(&base) {
                Some(full_path)
            } else {
                println!("Security check failed: full_path not under base");
                None // Tried to escape base_dir
            }
        }
        Err(e) => {
            println!("canonicalize() failed on {:?} -> {}", joined, e);
            None
        }
    }
}

pub fn render_home_with_file_list(
    index_path: &Path,
    upload_dir: &Path,
    web_prefix: &str,
) -> Option<String> {
    let template = fs::read_to_string(index_path).ok()?;
    let mut file_list_html = String::from("<ul>");

    if let Ok(entries) = fs::read_dir(upload_dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_path = format!("{}/{}", web_prefix.trim_end_matches('/'), file_name);
            file_list_html.push_str(&format!(
                "<li><a href=\"{}\">{}</a></li>",
                file_path, file_name
            ));
        }
    }

    file_list_html.push_str("</ul>");

    // Inject the list into the template
    let result = template.replace("::FILE_LIST::", &file_list_html);
    Some(result)
}
