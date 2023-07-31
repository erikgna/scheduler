use std::{io, fs};
use std::io::ErrorKind;
use std::path::{Path, PathBuf};
use rocket_multipart_form_data::FileField;
use tokio::fs as async_fs;
use std::thread::sleep;
use std::time::Duration;

pub async fn copy_file(from: &Path, to: &Path) -> io::Result<()> {
    async_fs::copy(from, to).await?;    

    async_fs::remove_file(from).await.map_err(|err| {
        if err.kind() == ErrorKind::NotFound {            
            return io::Error::new(ErrorKind::Other, "File already moved");
        }
        err
    })?;

    Ok(())
}

pub async fn save_file(upload_path: String, photo: Option<&Vec<FileField>>) -> Result<String, String> {    
    if let Some(file_fields) = photo {
        let file_field = file_fields.get(0).ok_or("No file in the photo field")?;

        let file_name = file_field.file_name.as_ref().map(|name| {
            let ext = Path::new(name).extension().and_then(std::ffi::OsStr::to_str).unwrap_or("png");
            format!("photo_{}.{}", chrono::Utc::now().timestamp(), ext.to_lowercase())
        }).unwrap_or_else(|| format!("photo_{}.png", chrono::Utc::now().timestamp()));        

        let dest_path = PathBuf::from(upload_path.clone()).join(&file_name);
        
        fs::create_dir_all(upload_path.clone()).map_err(|err| format!("Failed to create upload directory: {}", err))?;
        fs::copy(&file_field.path, &dest_path).map_err(|error| format!("Failed to copy the file: {}", error))?;

        sleep(Duration::from_secs(1));

        Ok(file_name)
    } else {
        Err("Photo field not found in form data".to_string())
    }
}

pub fn delete_file(filename: String) -> Result<(), String> {
    let path = Path::new(&filename);
    if path.exists() {
        let result = std::fs::remove_file(path);
        if result.is_err() {
            return Err("Error deleting file".to_string());
        } else {
            return Ok(());
        }
    } else {
        return Err("File does not exist.".to_string());
    }
}