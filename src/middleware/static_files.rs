use std::path::{Path, PathBuf};
use tokio::fs::File;
use tokio::io::AsyncReadExt;

use crate::http::{HttpRequest, HttpResponse};
use crate::router::RouteMatch;
use super::Next;

pub struct StaticFiles {
    root: PathBuf,
    prefix: String,
}

impl StaticFiles {
    pub fn new(root: impl Into<PathBuf>, prefix: impl Into<String>) -> Self {
        StaticFiles {
            root: root.into(),
            prefix: prefix.into(),
        }
    }


    pub async fn handle(&self, req: HttpRequest, route: RouteMatch, next: Next<'_>) -> HttpResponse {
        if !req.path.starts_with(&self.prefix) {
            return next.await;
        }

        let rel_path = req.path.trim_start_matches(&self.prefix);
        let file_path = self.root.join(rel_path.trim_start_matches('/'));

        match self.serve_file(&file_path).await {
            Ok(response) => response,
            Err(_) => next.await,
        }
    }

    async fn serve_file(&self, path: &Path) -> std::io::Result<HttpResponse> {
        let mut file = File::open(path).await?;
        let mut contents = Vec::new();
        file.read_to_end(&mut contents).await?;

        let content_type = mime_guess::from_path(path)
            .first_or_octet_stream()
            .to_string();

        let mut response = HttpResponse::new(200, "OK");
        response.add_header(
            "Content-Type".to_string(),
            content_type,
        );
        response.body = contents;

        Ok(response)
    }
}
