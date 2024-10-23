use std::path::{PathBuf, Path};
use uuid::Uuid;
use crate::error::{EditorError, Result};

#[derive(Debug, Clone)]
pub struct FileContent {
    pub path: PathBuf,
    pub content: String,
}

pub async fn pick_file() -> Result<FileContent> {
    let file_dialog = rfd::AsyncFileDialog::new();
    let file = file_dialog
      .set_title("Open a file")
      .pick_file()
      .await
      .ok_or(EditorError::FilePickerClosed)?;

    let path = file.path().to_owned();
    let content = tokio::fs::read_to_string(&path).await?;

    Ok(FileContent { path, content })
}