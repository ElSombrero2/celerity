use std::path::Path;

pub fn clone(remote_uri: String, branch: Option<String>, path: String) -> Option<String> {
    if let Some(branch) = branch {
        if let Ok(repository) = git2::build::RepoBuilder::new()
        .branch(&branch)
        .clone(&remote_uri, Path::new(&path)) {
            return Some(repository.path().to_string_lossy().to_string())
        }
    }
    Option::None
}