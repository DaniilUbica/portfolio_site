use rocket::{Data, get, post, tokio, uri};
use rocket::http::ContentType;
use rocket::response::{content, Redirect};
use std::io::Cursor;
use rocket::data::{ByteUnit, ToByteUnit};
use rocket::tokio::io::{AsyncReadExt, AsyncWriteExt};

const RESUME_FILE_PATH: &str = "./static/resume.pdf";
const FILE_LIMIT_SIZE: u64 = 10;

#[post("/admin/file_upload", data = "<data>")]
pub async fn upload_resume(content_type: &ContentType, data: Data<'_>) -> Result<Redirect, std::io::Error> {
    if content_type.is_form_data() {
        let mut file = tokio::fs::OpenOptions::new().write(true).truncate(true).open(RESUME_FILE_PATH).await?;
        let mut buffer = Vec::new();
        let mut stream = data.open(ByteUnit::from(FILE_LIMIT_SIZE).mebibytes());

        stream.read_to_end(&mut buffer).await?;

        let mut cursor = Cursor::new(buffer);
        tokio::io::copy(&mut cursor, &mut file).await?;

        file.flush().await?;

        Ok(Redirect::to(uri!(file_upload_success)))
    }
    else {
        Ok(Redirect::to(uri!(file_upload_error)))
    }
}

#[get("/file_upload_success")]
pub fn file_upload_success() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("../templates/file_upload_success.html"))
}

#[get("/file_upload_error")]
pub fn file_upload_error() -> content::RawHtml<&'static str> {
    content::RawHtml(include_str!("../templates/file_upload_error.html"))
}