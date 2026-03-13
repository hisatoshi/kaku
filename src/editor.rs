use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub fn open(editor: &str, path: &Path) -> Result<()> {
    // alternate screen buffer に切り替えてちらつきを防ぐ
    print!("\x1b[?1049h");
    let _ = std::io::Write::flush(&mut std::io::stdout());

    let result = Command::new(editor)
        .arg(path)
        .status()
        .with_context(|| format!("エディター '{}' の起動に失敗しました", editor));

    // エディター終了後にメインスクリーンを復元
    print!("\x1b[?1049l");
    let _ = std::io::Write::flush(&mut std::io::stdout());

    let status = result?;
    if !status.success() {
        anyhow::bail!(
            "エディターが異常終了しました (exit code: {:?})",
            status.code()
        );
    }

    Ok(())
}
