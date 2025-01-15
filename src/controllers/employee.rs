use axum::{
    routing::{get, post},
    Router,
};

pub struct FileService;

impl FileService {
    pub fn new() -> Router {
        Router::new()
            .route("/", post(FileService::upload_file))
            .route("/", get(FileService::retrieve_file))
    }
    pub async fn upload_file() {
        // retrieve the file bytes and store it in a `storage` folder
    }

    pub async fn retrieve_file() {
        // get the file by its id and return the bytes to user
    }
}
