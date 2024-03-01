use std::{fs, path::Path};
use git2::{Index, Repository};

pub mod github;

pub struct Git;

impl Git {
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
    
    pub fn reinit(to_remove: String, to_init: String) -> bool{
        let removing = fs::remove_dir_all(to_remove);
        if removing.is_ok() {
            if let Ok(repository) = Repository::init(to_init) {
                if let Ok(mut index) = repository.index() {
                    index.add_all(["."], git2::IndexAddOption::DEFAULT, None).unwrap();
                    index.write().unwrap();
                    if Git::commit(&repository, index, "Initial Commit") {
                        return true;
                    }
                }
            }
        }
        false
    }
    
    fn commit(repository: &Repository, mut index: Index, message: &str) -> bool {
        if let (Ok(oid), Ok(signature)) = (index.write_tree(), repository.signature() ) {
            if let Ok(tree) = repository.find_tree(oid) {
                return repository.commit(
                    Some("HEAD"),
                    &signature,
                    &signature,
                    message,
                    &tree,
                    &[],
                ).is_ok();
            }
        }
        false
    }   
}