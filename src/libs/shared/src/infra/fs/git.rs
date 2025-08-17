use crate::infra::system::validate_system_dependencies;
use crate::logging::GlobalLogger;
use std::process::Command;
use std::fs::{create_dir_all, read_dir};
use std::path::PathBuf;
use thiserror::Error;
use crate::infra::system::SystemError;

#[derive(Debug, Error)]
pub enum GitError {
    #[error("A system error occured when attempting to set up a Git repository: {0}")]
    SystemError(#[from] SystemError),

    #[error("Error cloning git repository: {0}")]
    Clone(String),

    #[error("Error pulling branch of git repository: {0}")]
    Pull(String),

    #[error("Git LFS error listing large files: {0}")]
    LfsList(String),

    #[error("Git LFS error pulling large files: {0}")]
    LfsPull(String),
}

#[derive(Clone)]
pub struct GitRepository {
    pub remote_url: String,
    pub remote_base_url: String,
    pub name: String,
}

/// TODO git lfs will pull down the large files when cloning. Using GIT_LFS_SKIP_SMUDGE=1
/// prevents that but subseuqnt Command(s) still run with that env value as 1 even when
/// explicitly removed or set to 0.
impl GitRepository {
    pub fn new(remote_base_url: String, name: String) ->  Self {
        Self {
            remote_url: Self::build_remote_url(remote_base_url.clone(), name.clone()),
            remote_base_url,
            name,
        }
    }

    fn build_remote_url(base_url: String, name: String) -> String {
        format!(
            "{}/{}{}",
            base_url.clone(),
            name.clone(),
            ".git"
        )
    }

    // Prepare the local environment for a clone or pull
    pub fn prepare(&self, params: PrepareRepositoryParams) -> Result<PreparedRepository, GitError> {
        let target_path = PathBuf::from(&params.target_dir);

        // Create the all of the directories at the target path. Works like  mkdir -p
        create_dir_all(&target_path)
            .map_err(|err| {
                GlobalLogger::error(format!("Error creating dirs '{:?}': {}", &target_path, err.to_string()).as_str());
                GitError::SystemError(SystemError::FileSystemError(format!("{}: {:?}", err.to_string(), &target_path)))
            })?;

        Ok(PreparedRepository::new(self.clone(), target_path))
    }
}

/// A locally prepared repository
pub struct PreparedRepository {
    pub repository: GitRepository,
    pub path: PathBuf
}

impl PreparedRepository {
    fn new(repo: GitRepository, path: PathBuf) -> Self {
        Self {
            repository: repo,
            path
        }
    }

    fn clone(&self, params: GitCloneParams) -> Result<&Self, GitError> {
        let mut cmd = Command::new("git");
        
        // Set the current work directory
        cmd.current_dir(&self.path);
        
        // Extend the headers on the clone command with the provided access token
        if let Some(access_token) = &params.access_token {
            cmd.arg("-c")
                .arg(format!("http.extraHeader=\"Authorization: Bearer {}\"", access_token));
        }

        // Add the branch to clone
        cmd.arg("clone");
        if let Some(branch) = params.branch{
            cmd.arg("--branch")
                .arg(branch)
                .arg("--single-branch");
        }

        cmd.arg(self.repository.remote_url.clone())
            .arg(".")
            .env("GIT_LFS_SKIP_SMUDGE", "1");

        // Run the command
        let output = cmd.output()
            .map_err(|err| {
                GlobalLogger::debug(format!("Error running `git clone`: {}", err.to_string()).as_str());
                GitError::Clone(err.to_string())
            })?;
        
        match output.status.code() {
            Some(code) => {
                if code == 0 {
                    return Ok(self)
                }
                
                return Err(
                    GitError::Clone(
                        String::from_utf8(output.stderr)
                            .unwrap_or("Git clone failed. Additionally, stderr from the git clone process could not be decoded".into())
                    )
                )
            },
            None => {
                return Err(GitError::Clone("The git clone operation was terminated by an unknown signal".into()))
            } 
        }
    }

    fn pull(&self, params: GitPullParams) -> Result<&Self, GitError> { 
        let mut cmd = Command::new("git");
        
        // Set the current work directory
        cmd.current_dir(&self.path);
        
        // Extend the headers on the clone command with the provided access token
        if let Some(access_token) = &params.access_token {
            cmd.arg("-c")
                .arg(format!("http.extraHeader=\"Authorization: Bearer {}\"", access_token));
        }

        cmd.arg("pull");
        
        // Add the branch to clone
        if let Some(branch) = params.branch {
            cmd.arg("origin")
                .arg(branch);
        }

        // Run the command
        cmd.output()
            .map_err(|err| {
                GlobalLogger::debug(format!("Error running `git pull`: {}", err.to_string()).as_str());
                GitError::Pull(err.to_string())
            })?;

        Ok(self)
    }

    /// Clones a git repository if it does not exist in the cache. Pull the repository
    /// if it does exist.
    pub fn clone_or_pull_repo(&self, params: GitCloneOrPullParams) -> Result<&Self, GitError> {
        let contains_files = match read_dir(self.path.clone()) {
            Ok(mut entries) => entries.next().is_some(), // Check if there's at least one entry
            Err(_) => false, // Path doesn't exist or isn't accessible
        };
        
        // Pull the repo at the target path if it exists otherwise clone
        if self.path.exists() && contains_files {
            self.pull(GitPullParams {
                branch: params.branch,
                access_token: params.access_token
            })
                .map_err(|err| err)?;

            return Ok(self)
        }

        self.clone(GitCloneParams {
            branch: params.branch,
            access_token: params.access_token
        })
            .map_err(|err| err)?;

        return Ok(self)
    }
}

pub struct PrepareRepositoryParams {
    pub target_dir: String
}

pub struct GitCloneOrPullParams {
    pub branch: Option<String>,
    pub access_token: Option<String>,
}

pub struct GitCloneParams {
    pub branch: Option<String>,
    pub access_token: Option<String>,
}

pub struct GitPullParams {
    pub branch: Option<String>,
    pub access_token: Option<String>,
}

pub struct GitLfsPullLargeFilesParams {
    pub access_token: Option<String>,
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>
}

pub struct GitLfsRepository {
    pub repo: PreparedRepository
}

impl GitLfsRepository {
    pub fn from_prepared_git_repo(repo: PreparedRepository) -> Result<Self, GitError> {
        validate_system_dependencies(vec!["git-lfs"])?;
        Ok(Self {
            repo
        })
    }

    pub fn pull(&self, params: GitLfsPullLargeFilesParams) -> Result<&Self, GitError> {
        let output = Command::new("git")
            .arg("lfs")
            .arg("ls-files")
            .arg("-n")
            .current_dir(&self.repo.path)
            .output()
            .map_err(|err| {
                GlobalLogger::debug(format!("Error running `git lfs ls-files`: {}", err.to_string()).as_str());
                GitError::LfsList(err.to_string())
            })?;

        let large_files: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .to_string()
            .lines()
            .map(String::from)
            .collect();

        GlobalLogger::debug(format!("Large files: '{:#?}' ", &large_files).as_str());

        let include_paths = params.include_paths.unwrap_or(Vec::new());
        let mut included_large_files_args: Vec<String> = Vec::new();
        for include_file in &include_paths {
            included_large_files_args.push(format!("--include=\"{}\"", include_file));
        }

        GlobalLogger::debug(format!("Files to include: {:#?}", &included_large_files_args).as_str());

        let exclude_paths = params.exclude_paths.unwrap_or(Vec::new());
        let mut excluded_large_files_args: Vec<String> = Vec::new();
        for exclude_file in &exclude_paths {
            excluded_large_files_args.push(format!("--exclude=\"{}\"", exclude_file));
        }

        GlobalLogger::debug(format!("Files to exclude: {:#?} ", &excluded_large_files_args).as_str());
        
        let mut cmd = Command::new("git");

        // Extend the headers on the clone command with the provided access token
        if let Some(access_token) = &params.access_token {
            cmd.arg("-c")
                .arg(format!("http.extraHeader=\"Authorization: Bearer {}\"", access_token));
        }
        
        cmd.arg("lfs")
            .arg("pull")
            .args(excluded_large_files_args)
            .args(included_large_files_args);
            // .env_remove("GIT_LFS_SKIP_SMUDGE")
            // .env("GIT_LFS_SKIP_SMUDGE", "0");

        cmd.output()
            .map_err(|err| {
                GlobalLogger::error(format!("Error running `git lfs pull`: {}", err.to_string()).as_str());
                GitError::LfsPull(err.to_string())
            })?;
        Ok(self)
    }
}

pub struct SyncGitRepositoryParams {
    pub name: String,
    pub remote_base_url: String,
    pub target_dir: String,
    pub branch: Option<String>,
    pub access_token: Option<String>,
}

pub struct SyncLfsRepositoryParams {
    pub name: String,
    pub remote_base_url: String,
    pub target_dir: String,
    pub branch: Option<String>,
    pub access_token: Option<String>,
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
}

pub trait SyncGitRepositoryImpl {
    fn sync_git_repo(&self, params: SyncGitRepositoryParams) -> Result<PreparedRepository, GitError>;
    fn sync_lfs_repo(&self, params: SyncLfsRepositoryParams) -> Result<GitLfsRepository, GitError>;
}

pub trait SyncGitRepository {}

impl<T: SyncGitRepository> SyncGitRepositoryImpl for T {
    fn sync_git_repo(&self, params: SyncGitRepositoryParams) -> Result<PreparedRepository, GitError> {
        // Initialize the git lfs repository
        let repo = GitRepository::new(
            params.remote_base_url,
            params.name
        );

        let prepared_repo = repo
            .prepare(PrepareRepositoryParams {
                target_dir: params.target_dir
            })?;

        prepared_repo.clone_or_pull_repo(GitCloneOrPullParams {
            branch: params.branch,
            access_token: params.access_token
        })?;

        Ok(prepared_repo)
    }

    fn sync_lfs_repo(&self, params: SyncLfsRepositoryParams) -> Result<GitLfsRepository, GitError> {
        let prepared_repo = self.sync_git_repo(SyncGitRepositoryParams {
            name: params.name.clone(),
            remote_base_url: params.remote_base_url.clone(),
            target_dir: params.target_dir.clone(),
            branch: params.branch.clone(),
            access_token: params.access_token.clone()
        })?;

        let git_lfs_repo = GitLfsRepository::from_prepared_git_repo(
            prepared_repo
        )?;
        
        git_lfs_repo.pull(GitLfsPullLargeFilesParams {
            access_token: params.access_token.clone(),
            include_paths: params.include_paths.clone(),
            exclude_paths: params.exclude_paths.clone()
        })?;

        Ok(git_lfs_repo)
    }
}