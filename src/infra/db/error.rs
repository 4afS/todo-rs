pub enum DbError {
    AlreadyExists,
    NotFound,
    InternalServerError(String),
}
