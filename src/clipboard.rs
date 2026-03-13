use anyhow::{Context, Result};

pub fn copy(text: &str) -> Result<()> {
    if is_wsl() {
        copy_via_powershell(text)
    } else {
        copy_via_arboard(text)
    }
}

fn copy_via_arboard(text: &str) -> Result<()> {
    let mut clipboard = arboard::Clipboard::new().context("クリップボードの初期化に失敗しました")?;
    clipboard
        .set_text(text)
        .context("クリップボードへの書き込みに失敗しました")?;
    Ok(())
}

fn copy_via_powershell(text: &str) -> Result<()> {
    use std::io::Write;
    use std::process::{Command, Stdio};

    // PowerShell の Set-Clipboard は UTF-8 を正しく扱える
    // stdin 経由で渡すことで引数のエスケープ問題を回避する
    let powershell_path = if std::path::Path::new("/mnt/c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe").exists() {
        "/mnt/c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe"
    } else {
        "powershell.exe"
    };

    let mut child = Command::new(powershell_path)
        .args([
            "-noprofile",
            "-command",
            "[Console]::InputEncoding = [System.Text.Encoding]::UTF8; $text = [Console]::In.ReadToEnd(); Set-Clipboard $text",
        ])
        .stdin(Stdio::piped())
        .spawn()
        .with_context(|| format!("{} の起動に失敗しました", powershell_path))?;

    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(text.as_bytes())
        .context("powershell.exe への書き込みに失敗しました")?;

    child
        .wait()
        .context("powershell.exe の終了待機に失敗しました")?;

    Ok(())
}

fn is_wsl() -> bool {
    std::fs::read_to_string("/proc/version")
        .map(|v| v.to_lowercase().contains("microsoft"))
        .unwrap_or(false)
}
