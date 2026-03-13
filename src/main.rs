mod clipboard;
mod config;
mod editor;

use anyhow::{Context, Result};
use chrono::Local;
use std::fs;

fn main() -> Result<()> {
    let config = config::Config::load()?;

    // ディレクトリ確認・作成
    fs::create_dir_all(&config.directory)
        .with_context(|| format!("ディレクトリの作成に失敗しました: {:?}", config.directory))?;

    // プロンプトファイル作成
    let filename = Local::now().format("%Y%m%d_%H%M%S.md").to_string();
    let prompt_path = config.directory.join(&filename);
    fs::write(&prompt_path, "")
        .with_context(|| format!("プロンプトファイルの作成に失敗しました: {:?}", prompt_path))?;

    // エディター起動・待機
    editor::open(&config.editor, &prompt_path)?;

    // ファイル内容を読み込んでクリップボードへコピー
    let content = fs::read_to_string(&prompt_path)
        .with_context(|| format!("ファイルの読み込みに失敗しました: {:?}", prompt_path))?;

    let trimmed = content.trim_end();
    clipboard::copy(trimmed)?;

    Ok(())
}
