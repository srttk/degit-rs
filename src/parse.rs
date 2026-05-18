use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Provider {
    GitHub,
    GitLab,
    Bitbucket,
}

#[derive(Debug, PartialEq)]
pub struct Repo {
    pub provider: Provider,
    pub user: String,
    pub name: String,
    pub refer: String,
}

impl Repo {
    pub fn provider_name(&self) -> &'static str {
        match self.provider {
            Provider::GitHub => "github",
            Provider::GitLab => "gitlab",
            Provider::Bitbucket => "bitbucket",
        }
    }

    pub fn parse(src: &str) -> Result<Repo, String> {
        let re = Regex::new(r"^(?:(?P<provider>github|gitlab|bitbucket):)?(?P<user>[^/]+)/(?P<repo>[^#]+)(?:#(?P<ref>.+))?$").map_err(|e| e.to_string())?;
        
        if let Some(caps) = re.captures(src) {
            let provider = match caps.name("provider").map(|m| m.as_str()) {
                Some("gitlab") => Provider::GitLab,
                Some("bitbucket") => Provider::Bitbucket,
                _ => Provider::GitHub,
            };
            let user = caps.name("user").unwrap().as_str().to_string();
            
            let mut repo_name = caps.name("repo").unwrap().as_str().to_string();
            if repo_name.ends_with(".git") {
                repo_name = repo_name.trim_end_matches(".git").to_string();
            }

            let refer = caps.name("ref")
                .map(|m| m.as_str().to_string())
                .unwrap_or_else(|| "HEAD".to_string());

            Ok(Repo {
                provider,
                user,
                name: repo_name,
                refer,
            })
        } else {
            Err(format!("Could not parse source '{}'", src))
        }
    }

    pub fn download_url(&self) -> String {
        match self.provider {
            Provider::GitHub => format!(
                "https://github.com/{}/{}/archive/{}.tar.gz",
                self.user, self.name, self.refer
            ),
            Provider::GitLab => format!(
                "https://gitlab.com/{}/{}/-/archive/{}/{}-{}.tar.gz",
                self.user, self.name, self.refer, self.name, self.refer
            ),
            Provider::Bitbucket => format!(
                "https://bitbucket.org/{}/{}/get/{}.tar.gz",
                self.user, self.name, self.refer
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_default() {
        let repo = Repo::parse("user/repo").unwrap();
        assert_eq!(repo.provider, Provider::GitHub);
        assert_eq!(repo.user, "user");
        assert_eq!(repo.name, "repo");
        assert_eq!(repo.refer, "HEAD");
    }

    #[test]
    fn test_parse_gitlab_with_ref() {
        let repo = Repo::parse("gitlab:user/repo#dev").unwrap();
        assert_eq!(repo.provider, Provider::GitLab);
        assert_eq!(repo.user, "user");
        assert_eq!(repo.name, "repo");
        assert_eq!(repo.refer, "dev");
    }

    #[test]
    fn test_parse_github_explicit() {
        let repo = Repo::parse("github:foo/bar.git").unwrap();
        assert_eq!(repo.provider, Provider::GitHub);
        assert_eq!(repo.user, "foo");
        assert_eq!(repo.name, "bar");
        assert_eq!(repo.refer, "HEAD");
    }
}
