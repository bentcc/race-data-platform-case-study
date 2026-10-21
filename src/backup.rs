//! Size and SHA-256 verification for files listed in a trusted manifest.

use std::fs::File;
use std::io::{self, Read};
use std::path::{Component, Path, PathBuf};

use sha2::{Digest, Sha256};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ManifestEntry {
    pub relative_path: PathBuf,
    pub size_bytes: u64,
    pub sha256: String,
}

impl ManifestEntry {
    /// Capture before transfer; keep this expected entry separately from the backup files.
    pub fn capture(root: &Path, relative_path: &Path) -> io::Result<Self> {
        if relative_path.as_os_str().is_empty()
            || relative_path
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "manifest paths must be relative without traversal components",
            ));
        }
        let root = root.canonicalize()?;
        let path = root.join(relative_path).canonicalize()?;
        if !path.starts_with(&root) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "manifest path escapes the backup root",
            ));
        }
        if !path.metadata()?.is_file() {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "manifest entries must name regular files",
            ));
        }
        // shortcut: use an offline, operator-controlled root; hostile writers require descriptor-relative opens.
        let mut file = File::open(path)?;
        let mut hasher = Sha256::new();
        let mut size_bytes = 0;
        let mut buffer = [0; 8192];
        loop {
            let count = file.read(&mut buffer)?;
            if count == 0 {
                break;
            }
            hasher.update(&buffer[..count]);
            size_bytes += count as u64;
        }
        Ok(Self {
            relative_path: relative_path.to_owned(),
            size_bytes,
            sha256: format!("{:x}", hasher.finalize()),
        })
    }
}

/// Verifies listed files only, against a nonempty manifest captured before transfer.
pub fn verify_backup(root: &Path, manifest: &[ManifestEntry]) -> io::Result<()> {
    if manifest.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "backup manifest must not be empty",
        ));
    }
    for expected in manifest {
        if ManifestEntry::capture(root, &expected.relative_path)? != *expected {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("backup mismatch: {}", expected.relative_path.display()),
            ));
        }
    }
    Ok(())
}
