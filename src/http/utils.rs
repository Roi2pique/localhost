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

pub fn generate_file_list_html(dir: &Path, web_prefix: &str) -> String {
    let mut html = String::from("<ul>");

    // println!("[DEBUG] Adding file: {:?} -> {}", dir, web_prefix);
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_path = format!("{}/{}", web_prefix.trim_end_matches('/'), file_name);
            html.push_str(&format!(
                "<li><a href=\"{}\">{}</a></li>",
                file_path, file_name
            ));
        }
    }
    println!("[DEBUG] Generated HTML: {}", html);
    html.push_str("</ul>");
    html
}

pub fn render_home_with_two_lists(
    index_path: &Path,
    upload_dir: &Path,
    script_dir: &Path,
    upload_prefix: &str,
    script_prefix: &str,
) -> Option<String> {
    let template = fs::read_to_string(index_path).ok()?;
    let upload_list = generate_file_list_html(upload_dir, upload_prefix);
    let script_list = generate_file_list_html(script_dir, script_prefix);

    let filled = template
        .replace("::FILE_LIST::", &upload_list)
        .replace("::SCRIPT_LIST::", &script_list);

    Some(filled)
}
