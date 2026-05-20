use crate::parse::{Provider, Repo};
use std::process::Command;

pub fn git_url(repo: &Repo) -> String {
    match repo.provider {
        Provider::GitHub => format!("https://github.com/{}/{}.git", repo.user, repo.name),
        Provider::GitLab => format!("https://gitlab.com/{}/{}.git", repo.user, repo.name),
        Provider::Bitbucket => format!("https://bitbucket.org/{}/{}.git", repo.user, repo.name),
    }
}

pub fn clone(repo: &Repo, dest: &str, verbose: bool) -> Result<(), String> {
    let url = git_url(repo);

    if verbose {
        println!("Cloning {} into {}", url, dest);
    }

    let mut args_clone = vec!["clone", &url, dest];
    if !verbose {
        args_clone.push("--quiet");
    }

    let status = Command::new("git")
        .args(&args_clone)
        .status()
        .map_err(|e| format!("Failed to spawn git: {}", e))?;

    if !status.success() {
        return Err(format!("git clone failed with status {:?}", status.code()));
    }

    if repo.refer != "HEAD" && repo.refer != "main" && repo.refer != "master" {
        if verbose {
            println!("Checking out ref: {}", repo.refer);
        }
        let checkout_status = Command::new("git")
            .arg("-C")
            .arg(dest)
            .args(&["checkout", &repo.refer])
            .status()
            .map_err(|e| format!("Failed to spawn git checkout: {}", e))?;

        if !checkout_status.success() {
            return Err("git checkout failed".to_string());
        }
    }

    let git_dir = std::path::Path::new(dest).join(".git");
    if git_dir.exists() {
        if let Err(e) = std::fs::remove_dir_all(&git_dir) {
            eprintln!("Warning: failed to remove .git folder: {}", e);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse::{Provider, Repo};

    #[test]
    fn test_git_url_github() {
        let repo = Repo {
            provider: Provider::GitHub,
            user: "Rich-Harris".to_string(),
            name: "degit".to_string(),
            refer: "HEAD".to_string(),
        };
        assert_eq!(git_url(&repo), "https://github.com/Rich-Harris/degit.git");
    }

    #[test]
    fn test_git_url_gitlab() {
        let repo = Repo {
            provider: Provider::GitLab,
            user: "Rich-Harris".to_string(),
            name: "degit".to_string(),
            refer: "HEAD".to_string(),
        };
        assert_eq!(git_url(&repo), "https://gitlab.com/Rich-Harris/degit.git");
    }
}
