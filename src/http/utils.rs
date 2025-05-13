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
