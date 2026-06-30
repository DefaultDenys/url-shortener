use crate::store::UrlRepository;

#[derive(Clone)]
pub struct AppState {
    pub url_repository: UrlRepository,
}

impl AppState {
    pub fn new(url_repository: UrlRepository) -> Self {
        Self { url_repository }
    }
}
