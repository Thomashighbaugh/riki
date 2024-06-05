// THIS IS THE FILE: src/wiki/history.rs

use git2::{Repository, Status, Commit, ObjectType, IndexAddOption, Tree, Signature};
use std::path::{Path, PathBuf};
use chrono::NaiveDateTime;
use std::fs::File;
use std::io::Write;

use crate::config::Config;
use crate::wiki::page::Wiki;

pub struct History {
    pub repo: Repository,
}

impl History {
    pub fn new(wiki: &Wiki) -> Result<History, git2::Error> {
        let repo = match Repository::open(&wiki.root_dir) {
            Ok(repo) => repo,
            Err(_) => Repository::init(&wiki.root_dir)?,
        };
        Ok(History { repo })
    }

    pub fn add_page(&self, page_name: &str, content: &str, config: &Config) -> Result<(), git2::Error> {
        let mut index = self.repo.index()?;
        let page_path = wiki.get_page_path(page_name);

        let mut file = File::create(page_path)?;
        file.write_all(content.as_bytes())?;

        index.add_path(Path::new(&page_path), IndexAddOption::DEFAULT)?;
        index.write()?;

        // Commit the changes
        let oid = index.write_tree()?;
        let sig = Signature::now("Riki", "riki@example.com")?;

        let commit_message = format!("Added page: {}", page_name);
        let parent_commit = self.repo.head()?.peel(ObjectType::Commit)?.id();
        self.repo.commit(
            Some(&commit_message),
            &sig,
            &sig,
            &parent_commit,
            &oid,
            &[],
        )?;

        Ok(())
    }

    pub fn commit_changes(&self, page_name: &str, content: &str, config: &Config) -> Result<(), git2::Error> {
        let mut index = self.repo.index()?;
        let page_path = wiki.get_page_path(page_name);

        let mut file = File::create(page_path)?;
        file.write_all(content.as_bytes())?;

        index.add_path(Path::new(&page_path), IndexAddOption::DEFAULT)?;
        index.write()?;

        // Commit the changes
        let oid = index.write_tree()?;
        let sig = Signature::now("Riki", "riki@example.com")?;

        let commit_message = format!("Updated page: {}", page_name);
        let parent_commit = self.repo.head()?.peel(ObjectType::Commit)?.id();
        self.repo.commit(
            Some(&commit_message),
            &sig,
            &sig,
            &parent_commit,
            &oid,
            &[],
        )?;

        Ok(())
    }

    pub fn get_page_history(&self, page_name: &str, config: &Config) -> Result<Vec<String>, git2::Error> {
        let page_path = wiki.get_page_path(page_name).strip_prefix(&wiki.root_dir)?;

        let mut history = Vec::new();

        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        let commits = revwalk.filter_map(|oid| {
            if let Ok(commit) = self.repo.find_commit(oid) {
                let commit_time = commit.time();
                let date = NaiveDateTime::from_timestamp(commit_time.seconds(), commit_time.nanos());

                if commit.tree()?.get_path(page_path)?.is_some() {
                    Some(format!("{} - {}", date.format("%Y-%m-%d %H:%M:%S"), commit.summary()))
                } else {
                    None
                }
            } else {
                None
            }
        }).collect();

        Ok(commits)
    }

    pub fn get_last_modified_date(&self, page_name: &str, config: &Config) -> Result<NaiveDateTime, git2::Error> {
        let page_path = wiki.get_page_path(page_name).strip_prefix(&wiki.root_dir)?;

        let mut revwalk = self.repo.revwalk()?;
        revwalk.push_head()?;
        let commits = revwalk.filter_map(|oid| {
            if let Ok(commit) = self.repo.find_commit(oid) {
                let commit_time = commit.time();
                let date = NaiveDateTime::from_timestamp(commit_time.seconds(), commit_time.nanos());

                if commit.tree()?.get_path(page_path)?.is_some() {
                    Some(date)
                } else {
                    None
                }
            } else {
                None
            }
        }).collect();

        if let Some(date) = commits.first() {
            Ok(date.clone())
        } else {
            Err(git2::Error::from_str("Page not found in Git history"))
        }
    }

    pub fn revert_page(&self, page_name: &str, commit_hash: &str, config: &Config) -> Result<(), git2::Error> {
        let page_path = wiki.get_page_path(page_name).strip_prefix(&wiki.root_dir)?;

        let commit = self.repo.find_commit(git2::Oid::from_str(commit_hash)?)?;

        let mut checkout_builder = self.repo.checkout_builder()?;
        checkout_builder.force();
        checkout_builder.path(page_path)?;
        checkout_builder.target(commit.id())?;
        checkout_builder.detach();
        checkout_builder.finish()?;

        Ok(())
    }
}
