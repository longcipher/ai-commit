use std::{fmt::Write, path::Path};

use anyhow::Result;
use git2::{DiffOptions, ErrorCode, Repository, Status, StatusOptions};

pub struct GitRepo {
    repo: Repository,
}

#[derive(Debug, Clone)]
pub struct GitStatus {
    pub staged: Vec<String>,
    pub modified: Vec<String>,
    pub untracked: Vec<String>,
}

impl GitRepo {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let repo = Repository::discover(path)?;
        Ok(Self { repo })
    }

    pub fn is_git_repo(&self) -> bool {
        !self.repo.is_bare()
    }

    pub fn get_status(&self) -> Result<GitStatus> {
        let mut status_options = StatusOptions::new();
        status_options
            .include_untracked(true)
            .include_ignored(false);

        let statuses = self.repo.statuses(Some(&mut status_options))?;

        let mut staged_files = Vec::new();
        let mut modified_files = Vec::new();
        let mut untracked_files = Vec::new();

        for status_entry in statuses.iter() {
            let status = status_entry.status();
            if let Some(path) = status_entry.path() {
                let path = path.to_string();

                if status.contains(Status::INDEX_NEW)
                    || status.contains(Status::INDEX_MODIFIED)
                    || status.contains(Status::INDEX_DELETED)
                    || status.contains(Status::INDEX_RENAMED)
                    || status.contains(Status::INDEX_TYPECHANGE)
                {
                    staged_files.push(path.clone());
                }

                if status.contains(Status::WT_MODIFIED)
                    || status.contains(Status::WT_DELETED)
                    || status.contains(Status::WT_TYPECHANGE)
                    || status.contains(Status::WT_RENAMED)
                {
                    modified_files.push(path.clone());
                }

                if status.contains(Status::WT_NEW) {
                    untracked_files.push(path);
                }
            }
        }

        Ok(GitStatus {
            staged: staged_files,
            modified: modified_files,
            untracked: untracked_files,
        })
    }

    pub fn get_status_porcelain(&self) -> Result<String> {
        let mut status_options = StatusOptions::new();
        status_options
            .include_untracked(true)
            .include_ignored(false);

        let statuses = self.repo.statuses(Some(&mut status_options))?;
        let mut output = String::new();

        for status_entry in statuses.iter() {
            let status = status_entry.status();
            if let Some(path) = status_entry.path() {
                let mut index_status = ' ';
                let mut worktree_status = ' ';

                // Index status
                if status.contains(Status::INDEX_NEW) {
                    index_status = 'A';
                } else if status.contains(Status::INDEX_MODIFIED) {
                    index_status = 'M';
                } else if status.contains(Status::INDEX_DELETED) {
                    index_status = 'D';
                } else if status.contains(Status::INDEX_RENAMED) {
                    index_status = 'R';
                } else if status.contains(Status::INDEX_TYPECHANGE) {
                    index_status = 'T';
                }

                // Worktree status
                if status.contains(Status::WT_NEW) {
                    worktree_status = '?';
                } else if status.contains(Status::WT_MODIFIED) {
                    worktree_status = 'M';
                } else if status.contains(Status::WT_DELETED) {
                    worktree_status = 'D';
                } else if status.contains(Status::WT_RENAMED) {
                    worktree_status = 'R';
                } else if status.contains(Status::WT_TYPECHANGE) {
                    worktree_status = 'T';
                }

                writeln!(output, "{index_status}{worktree_status} {path}")
                    .expect("Failed to write to string buffer");
            }
        }

        Ok(output)
    }

    pub fn stage_all(&self) -> Result<()> {
        let mut index = self.repo.index()?;
        index.add_all(std::iter::once(&"*"), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        Ok(())
    }

    pub fn stage_modified(&self) -> Result<()> {
        let mut index = self.repo.index()?;
        index.update_all(std::iter::once(&"*"), None)?;
        index.write()?;
        Ok(())
    }

    pub fn stage_untracked(&self) -> Result<()> {
        let mut index = self.repo.index()?;
        index.add_all(std::iter::once(&"*"), git2::IndexAddOption::DEFAULT, None)?;
        index.write()?;
        Ok(())
    }

    pub fn get_staged_diff(&self) -> Result<String> {
        // Check if we have any commits
        let has_commits = self.repo.head().is_ok();

        if has_commits {
            let head = self.repo.head()?.peel_to_tree()?;
            let mut index = self.repo.index()?;
            let index_tree = self.repo.find_tree(index.write_tree()?)?;

            let mut diff_options = DiffOptions::new();
            diff_options.context_lines(3);

            let diff = self.repo.diff_tree_to_tree(
                Some(&head),
                Some(&index_tree),
                Some(&mut diff_options),
            )?;

            let mut diff_output = String::new();
            diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
                match line.origin() {
                    '+' | '-' | ' ' => {
                        diff_output.push(line.origin());
                        diff_output.push_str(std::str::from_utf8(line.content()).unwrap_or(""));
                    }
                    _ => {}
                }
                true
            })?;

            Ok(diff_output)
        } else {
            // For initial commit, show index vs empty tree
            let mut index = self.repo.index()?;
            let index_tree = self.repo.find_tree(index.write_tree()?)?;
            let empty_tree = self.repo.treebuilder(None)?.write()?;
            let empty_tree_obj = self.repo.find_tree(empty_tree)?;

            let mut diff_options = DiffOptions::new();
            diff_options.context_lines(3);

            let diff = self.repo.diff_tree_to_tree(
                Some(&empty_tree_obj),
                Some(&index_tree),
                Some(&mut diff_options),
            )?;

            let mut diff_output = String::new();
            diff.print(git2::DiffFormat::Patch, |_delta, _hunk, line| {
                match line.origin() {
                    '+' | '-' | ' ' => {
                        diff_output.push(line.origin());
                        diff_output.push_str(std::str::from_utf8(line.content()).unwrap_or(""));
                    }
                    _ => {}
                }
                true
            })?;

            Ok(diff_output)
        }
    }

    pub fn commit(&self, message: &str) -> Result<String> {
        let signature = self.repo.signature()?;
        let mut index = self.repo.index()?;
        let tree_id = index.write_tree()?;
        let tree = self.repo.find_tree(tree_id)?;

        let parent_commit = match self.repo.head() {
            Ok(head) => Some(head.peel_to_commit()?),
            Err(ref e) if e.code() == ErrorCode::UnbornBranch => None,
            Err(e) => return Err(e.into()),
        };

        let parents: Vec<&git2::Commit> = parent_commit.iter().collect();

        let commit_id = self.repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &parents,
        )?;

        Ok(commit_id.to_string())
    }
}
