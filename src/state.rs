use crate::store::{ClickRepository, UrlRepository};

#[derive(Clone)]
pub struct AppState {
    pub url_repository: UrlRepository,
    pub click_repository: ClickRepository,
}

impl AppState {
    pub fn new(url_repository: UrlRepository, click_repository: ClickRepository) -> Self {
        Self {
            url_repository,
            click_repository,
        }
    }
}
