use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use axum::{
    http::StatusCode,
    Json
};

use serde::{Deserialize, Serialize};

pub async fn save_file(
    Json(payload): Json<SaveFileRequest>,
) -> impl axum::response::IntoResponse {
    // Save the content to my_file.txt
    match File::create("my_file.txt").await {
        Ok(mut file) => {
            match file.write_all(payload.content.as_bytes()).await {
                Ok(_) => {
                    match file.flush().await {
                        Ok(_) => {
                            let response = SaveFileResponse {
                                success: true,
                                message: "File saved successfully".to_string(),
                                filename: "my_file.txt".to_string(),
                            };
                            (StatusCode::OK, Json(response))
                        }
                        Err(e) => {
                            let response = SaveFileResponse {
                                success: false,
                                message: format!("Failed to flush file: {}", e),
                                filename: "my_file.txt".to_string(),
                            };
                            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
                        }
                    }
                }
                Err(e) => {
                    let response = SaveFileResponse {
                        success: false,
                        message: format!("Failed to write to file: {}", e),
                        filename: "my_file.txt".to_string(),
                    };
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
                }
            }
        }
        Err(e) => {
            let response = SaveFileResponse {
                success: false,
                message: format!("Failed to create file: {}", e),
                filename: "my_file.txt".to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(response))
        }
    }
}

// the input to our `save_file` handler
#[derive(Deserialize)]
pub struct SaveFileRequest {
    pub content: String,
}

// the output to our `save_file` handler
#[derive(Serialize)]
pub struct SaveFileResponse {
    pub success: bool,
    pub message: String,
    pub filename: String,
}