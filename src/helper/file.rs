use axum::extract::Multipart;
use chrono::Utc;
use tokio::{fs::File, io::AsyncWriteExt};

pub async fn upload_file(mut multipart: Multipart) -> Result<String, String> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field_name = field.name().unwrap().to_string();
        if field_name == "attachment" {
            let filename = field.file_name().map(|name| name.to_string()).unwrap();
            let data = field.bytes().await.unwrap();
            let img_name = Utc::now().timestamp().to_string()+"_"+&filename;

            let mut file = File::create(format!("./public/uploads/{}", img_name)).await.unwrap();
            file.write(&data).await.unwrap();
            return Ok(format!("./public/uploads/{}", img_name));
        }
    }
    Err("Missing required filed: attachment".to_string())
}