use flate2::read::GzDecoder;
use std::fs::File;
use std::path::{Path, PathBuf};
use tar::Archive;

pub fn extract_tarball(tar_path: &Path, dest: &Path, force: bool) -> Result<(), String> {
    let file = File::open(tar_path).map_err(|e| e.to_string())?;
    let tar = GzDecoder::new(file);
    let mut archive = Archive::new(tar);

    for entry in archive.entries().map_err(|e| e.to_string())? {
        let mut entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path().map_err(|e| e.to_string())?.into_owned();

        let mut components = path.components();
        components.next(); // strip root dir of the tarball

        let stripped_path = components.as_path().to_path_buf();
        if stripped_path.as_os_str().is_empty() {
            continue; // It's just the root dir itself
        }

        let target = dest.join(&stripped_path);

        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }

        if target.exists() && !force {
            return Err(format!(
                "{} already exists. Use --force to overwrite.",
                target.display()
            ));
        }

        entry.unpack(&target).map_err(|e| e.to_string())?;
    }

    Ok(())
}
