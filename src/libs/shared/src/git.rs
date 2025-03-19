use crate::errors::Error;
use crate::system::{validate_system_dependencies, Env};
use crate::logging::GlobalLogger;
use std::process::Command;
use std::fs::{create_dir_all, read_dir};
use std::path::PathBuf;

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
    pub fn prepare(&self, params: PrepareRepositoryParams) -> Result<PreparedRepository, Error> {
        // Grab the shared env utility. The shared_data_dir will be used as part
        // of the clone target directory
        let shared_data_dir = Env::new()
            .map_err(|err| err)? 
            .shared_data_dir;

        let target_path = PathBuf::from(
            format!(
                "{}/{}/{}/{}",
                &shared_data_dir,
                &params.target_dir_prefix,
                &self.remote_base_url.replace("://", "/"),
                &self.name
            )
        );

        GlobalLogger::debug(format!("Preparing target path: {:?}", &target_path).as_str());

        // Create the all of the directories in the download_dir path. Works like
        // mkdir -p. Propogate the error if any
        create_dir_all(&target_path)
            .map_err(|err| {
                GlobalLogger::error(format!("Error creating dirs '{:?}': {}", &target_path, err.to_string()).as_str());
                Error::new(format!("{}: {:?}", err.to_string(), &target_path))
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

    fn clone(&self, params: GitCloneParams) -> Result<&Self, Error> {
        GlobalLogger::debug(format!("Cloning to path: {:?}", &self.path).as_str());
        
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
            .arg(".");
            // .env("GIT_LFS_SKIP_SMUDGE", "1");

        // Run the command
        cmd.output()
            .map_err(|err| {
                GlobalLogger::debug(format!("Error running `git clone`: {}", err.to_string()).as_str());
                Error::new(err.to_string())
            })?;

        Ok(self)
    }

    fn pull(&self, params: GitPullParams) -> Result<&Self, Error> {
        GlobalLogger::debug(format!("Pulling repo at path '{:?}'", &self.path).as_str());
        
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
                Error::new(err.to_string())
            })?;

        Ok(self)
    }

    /// Clones a git repository if it does not exist in the cache. Pull the repository
    /// if it does exist.
    pub fn clone_or_pull_repo(&self, params: GitCloneOrPullParams) -> Result<&Self, Error> {
        let contains_files = match read_dir(self.path.clone()) {
            Ok(mut entries) => entries.next().is_some(), // Check if there's at least one entry
            Err(_) => false, // Path doesn't exist or isn't accessible
        };
        
        // Pull the at the target path if it exists otherwise clone
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
    pub target_dir_prefix: String
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
    pub fn from_prepared_git_repo(repo: PreparedRepository) -> Result<Self, Error> {
        validate_system_dependencies(vec!["git-lfs"])?;
        Ok(Self {
            repo
        })
    }

    pub fn pull(&self, params: GitLfsPullLargeFilesParams) -> Result<&Self, Error> {
        let output = Command::new("git")
            .arg("lfs")
            .arg("ls-files")
            .arg("-n")
            .current_dir(&self.repo.path)
            .output()
            .map_err(|err| {
                GlobalLogger::debug(format!("Error running `git lfs ls-files`: {}", err.to_string()).as_str());
                Error::new(err.to_string())
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

        GlobalLogger::debug(format!("Running: {:?}", &cmd).as_str());

        cmd.output()
            .map_err(|err| {
                GlobalLogger::error(format!("Error running `git lfs pull`: {}", err.to_string()).as_str());
                Error::new(err.to_string())
            })?;

        GlobalLogger::debug("git lfs pull successful");

        Ok(self)
    }
}

pub struct SyncGitRepositoryParams {
    pub name: String,
    pub remote_base_url: String,
    pub target_dir_prefix: String,
    pub branch: Option<String>,
    pub access_token: Option<String>,
}

pub struct SyncLfsRepositoryParams {
    pub name: String,
    pub remote_base_url: String,
    pub target_dir_prefix: String,
    pub branch: Option<String>,
    pub access_token: Option<String>,
    pub include_paths: Option<Vec<String>>,
    pub exclude_paths: Option<Vec<String>>,
}

pub trait SyncGitRepositoryImpl {
    fn sync_git_repo(&self, params: SyncGitRepositoryParams) -> Result<PreparedRepository, Error>;
    fn sync_lfs_repo(&self, params: SyncLfsRepositoryParams) -> Result<GitLfsRepository, Error>;
}

pub trait SyncGitRepository {}

impl<T: SyncGitRepository> SyncGitRepositoryImpl for T {
    fn sync_git_repo(&self, params: SyncGitRepositoryParams) -> Result<PreparedRepository, Error> {
        // Initialize the git lfs repository
        let repo = GitRepository::new(
            params.remote_base_url,
            params.name
        );

        let prepared_repo = repo
            .prepare(PrepareRepositoryParams {
                target_dir_prefix: params.target_dir_prefix
            })?;

        prepared_repo.clone_or_pull_repo(GitCloneOrPullParams {
            branch: params.branch,
            access_token: params.access_token
        })?;

        Ok(prepared_repo)
    }

    fn sync_lfs_repo(&self, params: SyncLfsRepositoryParams) -> Result<GitLfsRepository, Error> {
        let prepared_repo = self.sync_git_repo(SyncGitRepositoryParams {
            name: params.name.clone(),
            remote_base_url: params.remote_base_url.clone(),
            target_dir_prefix: params.target_dir_prefix.clone(),
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