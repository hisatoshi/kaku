use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub fn open(editor: &str, path: &Path) -> Result<()> {
    let status = Command::new(editor)
        .arg(path)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status()
        .with_context(|| format!("エディター '{}' の起動に失敗しました", editor))?;

    if !status.success() {
        anyhow::bail!(
            "エディターが異常終了しました (exit code: {:?})",
            status.code()
        );
    }

    Ok(())
}
