use crate::errors::Error;
use crate::utils::{validate_system_dependencies, Env};
use crate::clients::ClientError;
use crate::logging::GlobalLogger;
use std::process::Command;
use std::fs::create_dir_all;

#[derive(Eq, PartialEq)]
pub enum ServiceContext {
    Models,
    Datasets
}

pub struct GitClient {}

impl GitClient {
    pub fn new() -> Result<GitClient, Error> {
        validate_system_dependencies(vec!["git"])?;
        Ok(Self {})
    }

    /// Clones a git repository if it does not exist in the cache. Pull the repository
    /// if it does exist.
    pub fn clone_or_pull_repo(&self, repo_base_url: String, repo_name: String, context: ServiceContext, _access_token: Option<String>) -> Result<String, ClientError> {
        let url = format!(
            "{}/{}",
            &repo_base_url,
            repo_name
        );

        GlobalLogger::debug(format!("Url to clone: {}", &url).as_str());

        // Grab the shared env utility. The shared_data_dir will be used as part
        // of the clone target directory
        let shared_data_dir = Env::new()
            .map_err(|err| ClientError::new(err.to_string()))? 
            .shared_data_dir;

        // Local path to the clone target directory
        let context_dir = if context == ServiceContext::Models { "models" } else { "datasets" };
        let cloned_dir = format!(
            "{}/{}/{}/{}",
            &shared_data_dir,
            context_dir,
            &repo_base_url.replace("://", "/"),
            &repo_name
        );

        GlobalLogger::debug(format!("Cloning into directory: {}", &cloned_dir).as_str());

        // Create the all of the directories in the download_dir path. Works like
        // mkdir -p. Propogate the error if any
        create_dir_all(&cloned_dir)
            .map_err(|err| {
                GlobalLogger::debug(format!("Error creating dirs '{}': {}", &cloned_dir, err.to_string()).as_str());
                ClientError::new(format!("{}: {}", err.to_string(), &cloned_dir))
            })?;

        let clone_path = std::path::Path::new(&cloned_dir);

        GlobalLogger::debug(format!("Clone path: {:#?}", &cloned_dir).as_str());

        let repo = gix::prepare_clone(url.clone(), clone_path)
            .map_err(|err| {
                let msg = format!("Failed to clone repository: {}", err.to_string());
                GlobalLogger::error(&msg.as_str());
                ClientError::new(String::from(&msg))
            })?
            .persist();

        GlobalLogger::debug(format!("Repository {:#?}", &repo).as_str());

        Ok(cloned_dir)
    }
}

pub struct GitLfsClient {
    git_client: GitClient
}

impl GitLfsClient {
    pub fn new() -> Result<GitLfsClient, Error> {
        validate_system_dependencies(vec!["git-lfs"])?;
        let git_client = GitClient::new()?;
        Ok(Self {
            git_client
        })
    }

    pub fn pull_large_files(
        &self, repo_base_url: String,
        repo_name: String,
        context: ServiceContext,
        access_token: Option<String>,
        files: Option<Vec<String>>
    ) -> Result<String, ClientError> {
        let cloned_dir = self.git_client.clone_or_pull_repo(
            repo_base_url,
            repo_name.clone(),
            context,
            access_token
        )?;

        GlobalLogger::debug(format!("Cloned repoistory '{}' to {}", &repo_name, &cloned_dir).as_str());

        let output = Command::new("git-lfs")
            .arg("lfs")
            .arg("ls-files")
            .current_dir(&cloned_dir)
            .output()
            .map_err(|err| {
                GlobalLogger::debug(format!("Error running `git lfs ls-files`: {}", err.to_string()).as_str());
                ClientError::new(err.to_string())
            })?;

        let large_files: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .to_string()
            .lines()
            .map(String::from)
            .collect();

        GlobalLogger::debug(format!("Large files to pull '{:#?}' ", &large_files).as_str());

        let mut excluded_large_files_args: Vec<String> = Vec::new();
        let files = files.unwrap_or(Vec::new());
        if files.len() > 0 {
            for large_file in large_files {
                if !files.contains(&large_file) { 
                    excluded_large_files_args.push(format!("--exclude=\"{}\"", large_file));
                };
            }
        }

        GlobalLogger::debug(format!("Files to exclude '{:#?}' ", &excluded_large_files_args).as_str());
        
        Command::new("git")
            .arg("lfs")
            .arg("pull")
            .args(excluded_large_files_args)
            .output()
            .map_err(|err| {
                GlobalLogger::error(format!("Error running `git lfs pull`: {}", err.to_string()).as_str());
                ClientError::new(err.to_string())
            })?;

        Ok(cloned_dir)
    }
}