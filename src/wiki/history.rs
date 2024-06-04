// src/wiki/history.rs

use git2::{Repository, Status, Commit, ObjectType, IndexAddOption, Tree, Signature};
use std::path::{Path, PathBuf};

pub struct Wiki {
    // ... other fields
}

impl Wiki {
    // ... other methods 

    // Function to initialize a Git repository in the wiki directory
    fn init_git_repo(&self) -> Result<Repository, Box<dyn std::error::Error>> {
        let repo = match Repository::open(&self.root_dir) {
            Ok(repo) => repo, 
            Err(_) => Repository::init(&self.root_dir)?, 
        };
        Ok(repo)
    }

    // Function to stage and commit changes
    fn commit_changes(&self, repo: &Repository, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut index = repo.index()?;
        index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;

        let tree_id = index.write_tree()?; 
        let tree = repo.find_tree(tree_id)?;

        // Find the last commit as the parent commit
        let head = repo.head()?;
        let parent = if head.is_branch() {
            repo.find_commit(head.target().unwrap())?
        } else {
            return Err("No commit history found".into());
        };

        let sig = repo.signature()?;
        repo.commit(Some("HEAD"), &sig, &sig, message, &tree, &[&parent])?; 
        Ok(())
    }

    // Function to view history of a page
    pub fn view_page_history(&self, page_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let repo = self.init_git_repo()?;
        let page_path = self.get_page_path(page_name).strip_prefix(&self.root_dir)?;
        let page_path_str = page_path.to_str().ok_or("Invalid page path")?; 

        let mut revwalk = repo.revwalk()?; 
        revwalk.push_head()?; 
        revwalk.set_sorting(git2::Sort::TIME)?;

        println!("History for '{}':", page_name);
        for commit_id in revwalk {
            let commit_id = commit_id?;
            let commit = repo.find_commit(commit_id)?;
            if commit.tree()?.get_path(page_path)?.is_some() {
                println!("- {}: {}", commit_id, commit.message().unwrap_or("")); 
            }
        }
        Ok(())
    }

    // Function to revert a page to a specific commit
    pub fn revert_page(&self, page_name: &str, commit_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let repo = self.init_git_repo()?;
        let obj = repo.revparse_single(commit_id)?; 

        if let Some(commit) = obj.as_commit() {
            let page_path = self.get_page_path(page_name).strip_prefix(&self.root_dir)?;
            let page_path_str = page_path.to_str().ok_or("Invalid page path")?; 

            let tree = commit.tree()?; 
            let blob = tree.get_path(page_path)?; 

            if let Some(blob) = blob {
                let content = blob.content()?;
                let mut file = File::create(self.get_page_path(page_name))?;
                file.write_all(content)?; 
                println!("Page '{}' reverted to commit '{}'.", page_name, commit_id);
                Ok(())
            } else {
                Err(format!("Page '{}' not found in commit '{}'.", page_name, commit_id).into())
            }
        } else {
            Err(format!("Invalid commit ID: '{}'", commit_id).into())
        }
    }

    // ... other methods
}

// ... helper functions (create_or_load_index, get_schema_fields, get_snippet, sanitize_tag, etc.)
