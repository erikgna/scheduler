use std::io;
use std::io::ErrorKind;
use std::path::Path;
use rocket_multipart_form_data::FileField;
use tokio::fs as async_fs;

const UPLOAD_DIR: &str = "./uploads/";
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

pub async fn save_file(photo: Option<&Vec<FileField>>) -> String {
    if let Some(file_fields) = photo {
        let file_field = &file_fields[0];
        
        let file_name = &file_field.file_name;
        let path = &file_field.path;

        let file_extension = match file_name {
            Some(name) => {
                let ext = Path::new(name)
                    .extension()
                    .and_then(std::ffi::OsStr::to_str)
                    .unwrap_or("png");
                ext.to_lowercase()
            }
            None => "png".to_string(),
        };

        let new_file_name = format!("photo_{}.{}", chrono::Utc::now().timestamp(), file_extension);
        let dest_path = Path::new(UPLOAD_DIR).join(&new_file_name);

        match copy_file(path, &dest_path).await {
            Ok(_) => {                
                return dest_path.to_string_lossy().to_string();
            }
            Err(error) => {                
                
                    return format!("Failed to copy the file: {}", error);       
            }
        }
    } else {                
            return "Photo field not found in form data".to_string();        
    }
}