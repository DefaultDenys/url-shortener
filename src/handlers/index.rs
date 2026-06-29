use axum::response::Html;

pub async fn index_handler() -> Html<&'static str> {
    tracing::debug!("serving index page");
    Html(include_str!("../../templates/index.html"))
}
