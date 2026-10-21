use std::path::Path;

use race_data_platform_examples::backup::{verify_backup, ManifestEntry};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures/synthetic-backup");
    let manifest = [ManifestEntry {
        relative_path: "events.csv".into(),
        size_bytes: 53,
        sha256: "85d8baa86d925db301e77f70d1b4934ba5e129370b18295bd02b236170e39b56".into(),
    }];
    verify_backup(&root, &manifest)?;
    println!("Verified the synthetic backup against its recorded size and SHA-256.");
    Ok(())
}
