use std::path::Path;
use tokio::fs::File;
use tokio_util::codec::{BytesCodec, FramedRead};
use reqwest::multipart;

/// Streams target endpoint forensic media captures safely to the Cloud Run Node engine.
pub async fn upload_forensic_media(
    api_url: &str,
    file_path: &str,
    bearer_token: &str,
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let path = Path::new(file_path);
    if !path.exists() {
        return Err(format!("File System Anomaly: Path '{:?}' unreadable.", path).into());
    }

    // Open local capture file without locking the asynchronous execution thread
    let file = File::open(path).await?;
    let file_name = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unnamed_payload")
        .to_string();

    // Dynamically derive the media mime mapping
    let mime_type = mime_guess::from_path(path)
        .first_raw()
        .unwrap_or("application/octet-stream");

    // Chunk the binary array into an optimized network frame reader
    let stream_reader = FramedRead::new(file, BytesCodec::new());
    let wrapped_body = reqwest::Body::wrap_stream(stream_reader);

    // Map multipart payload field names precisely matching the multer server layer rules
    let media_field = multipart::Part::stream(wrapped_body)
        .file_name(file_name)
        .mime_str(mime_type)?;

    let upload_form = multipart::Form::new().part("media", media_field);

    // Dispatch payload securely over network gateway
    let HTTP_client = reqwest::Client::new();
    let gateway_response = HTTP_client
        .post(api_url)
        .bearer_auth(bearer_token)
        .multipart(upload_form)
        .send()
        .await?;

    if gateway_response.status().is_success() {
        let execution_log = gateway_response.text().await?;
        Ok(execution_log)
    } else {
        let error_status = gateway_response.status();
        Err(format!("Cloud Engine Transaction Rejected [HTTP {}]", error_status).into())
    }
}
