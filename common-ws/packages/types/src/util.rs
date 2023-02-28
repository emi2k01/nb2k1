use std::path::PathBuf;

/// Looks for a file in the current directory and its ancestors.
///
/// If the file is found, it returns its path, otherwise returns `None`.
pub(crate) fn find_file_upwards(name: &str) -> Option<PathBuf> {
    let mut current_dir = Some(PathBuf::from(".").canonicalize().unwrap());
    while current_dir.is_some() {
        let would_be_path = current_dir.as_ref().unwrap().join(name);
        let file_exists = would_be_path.exists();
        if file_exists {
            return Some(would_be_path);
        }
        current_dir = current_dir.unwrap().parent().map(|path| path.to_path_buf());
    }
    None
}
