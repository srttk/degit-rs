mod extract;
mod fetch;
mod git;
mod parse;

use clap::{Parser, ValueEnum};
use fetch::Fetcher;
use parse::Repo;
use std::path::Path;

#[derive(Clone, Debug, ValueEnum, PartialEq)]
pub enum Mode {
    Tar,
    Git,
}

#[derive(Parser, Debug)]
#[command(name = "degit", about = "degit in rust", version)]
pub struct Args {
    /// The source repository
    #[arg(required_unless_present = "clear_cache")]
    pub src: Option<String>,

    /// The destination directory
    #[arg(default_value = ".")]
    pub dest: String,

    /// Force overwrite existing files
    #[arg(short, long)]
    pub force: bool,

    /// Use local cache
    #[arg(short, long)]
    pub cache: bool,

    /// Clear global cache
    #[arg(long)]
    pub clear_cache: bool,

    /// Verbose output
    #[arg(short, long)]
    pub verbose: bool,

    /// Download mode (tar or git)
    #[arg(short, long, value_enum, default_value_t = Mode::Tar)]
    pub mode: Mode,
}

fn main() {
    let args = Args::parse();

    let fetcher = Fetcher::new();

    if args.clear_cache {
        if let Err(e) = fetcher.clear_cache() {
            eprintln!("Failed to clear cache: {}", e);
            std::process::exit(1);
        } else if args.verbose {
            println!("Cache cleared.");
        }
        if args.src.is_none() {
            return;
        }
    }

    if args.src.is_none() {
        eprintln!("Error: missing source repository.");
        std::process::exit(1);
    }
    let src = args.src.unwrap();

    if args.mode == Mode::Git {
        match Repo::parse(&src) {
            Ok(repo) => {
                if let Err(e) = git::clone(&repo, &args.dest, args.verbose) {
                    eprintln!("Error during git clone: {}", e);
                    std::process::exit(1);
                }
                if args.verbose {
                    println!("Successfully cloned and setup repository.");
                }
            }
            Err(e) => {
                eprintln!("Error parsing source: {}", e);
                std::process::exit(1);
            }
        }
        return;
    }

    match Repo::parse(&src) {
        Ok(repo) => {
            if args.verbose {
                println!("Parsed repo: {:?}", repo);
                println!("Download URL: {}", repo.download_url());
            }

            match fetcher.fetch(&repo, args.cache, args.verbose) {
                Ok(tarball_path) => {
                    let dest_path = Path::new(&args.dest);
                    if args.verbose {
                        println!("Extracting {:?} to {:?}", tarball_path, dest_path);
                    }
                    if let Err(e) = extract::extract_tarball(&tarball_path, dest_path, args.force) {
                        eprintln!("Error during extraction: {}", e);
                        std::process::exit(1);
                    }
                    if args.verbose {
                        println!("Successfully downloaded and extracted.");
                    }
                }
                Err(e) => {
                    eprintln!("Error fetching tarball: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error parsing source: {}", e);
            std::process::exit(1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_args_default() {
        let args = Args::try_parse_from(&["degit", "user/repo"]).unwrap();
        assert_eq!(args.src, Some("user/repo".to_string()));
        assert_eq!(args.dest, ".");
        assert_eq!(args.cache, false);
        assert_eq!(args.mode, Mode::Tar);
    }

    #[test]
    fn test_cli_args_full() {
        let args = Args::try_parse_from(&[
            "degit",
            "user/repo",
            "my-dir",
            "--force",
            "--cache",
            "--mode",
            "git",
        ])
        .unwrap();
        assert_eq!(args.src, Some("user/repo".to_string()));
        assert_eq!(args.dest, "my-dir");
        assert_eq!(args.force, true);
        assert_eq!(args.cache, true);
        assert_eq!(args.mode, Mode::Git);
    }
}
