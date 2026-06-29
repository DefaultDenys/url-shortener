use uuid::Uuid;

pub fn generate_url_short() -> String {
    Uuid::new_v4().to_string()
}
