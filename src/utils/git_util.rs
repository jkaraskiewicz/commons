use std::path::PathBuf;

use git2::{IndexAddOption, Repository};

use crate::types::errors::CommonsError;

pub fn repository_exists(dir_path: &PathBuf) -> Result<bool, CommonsError> {
    let repository = get_repository(dir_path);
    Ok(repository.is_ok())
}

pub fn initialize_repository(dir_path: &PathBuf) -> Result<Repository, CommonsError> {
    let repository = Repository::init(dir_path)?;
    Ok(repository)
}

pub fn get_repository(dir_path: &PathBuf) -> Result<Repository, CommonsError> {
    let repository = Repository::open(dir_path)?;
    Ok(repository)
}

pub fn force_get_repository(dir_path: &PathBuf) -> Result<Repository, CommonsError> {
    get_repository(dir_path).or_else(|_| initialize_repository(dir_path))
}

pub fn add_all(repository: &Repository) -> Result<(), CommonsError> {
    let mut index = repository.index()?;
    index.add_all(&["*"], IndexAddOption::FORCE, None)?;
    index.write()?;
    Ok(())
}

pub fn is_working_copy_clean(repository: &Repository) -> Result<bool, CommonsError> {
    let index = repository.index()?;
    Ok(index.is_empty())
}

pub fn commit(repository: &Repository, message: &str) -> Result<(), CommonsError> {
    let mut index = repository.index()?;
    let oid = index.write_tree()?;
    let signature = repository.signature()?;
    let parent_commit = repository.head()?.peel_to_commit()?;
    let tree = repository.find_tree(oid)?;
    repository.commit(
        Some("HEAD"),
        &signature,
        &signature,
        message,
        &tree,
        &[&parent_commit],
    )?;
    Ok(())
}

pub fn create_branch(repository: &Repository, name: &str) -> Result<(), CommonsError> {
    let head_commit = repository.head()?.peel_to_commit()?;
    repository.branch(name, &head_commit, false)?;
    Ok(())
}

pub fn checkout_branch(repository: &Repository, name: &str) -> Result<(), CommonsError> {
    let branch = repository.find_branch(name, git2::BranchType::Local)?;
    let commit = branch.get().peel(git2::ObjectType::Commit)?;
    repository.checkout_tree(&commit, None)?;
    repository.set_head(branch.get().name().unwrap())?;
    Ok(())
}

pub fn list_branches(repository: &Repository) -> Result<Vec<String>, CommonsError> {
    let branches = repository.branches(Some(git2::BranchType::Local))?;
    let result: Vec<String> = branches
        .map(|branch| {
            let branch = branch.unwrap().0;
            branch.get().name().unwrap().to_string()
        })
        .collect();
    Ok(result)
}
