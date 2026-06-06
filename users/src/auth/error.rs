#[derive(Debug)]
pub enum AuthError {
    UserAlreadyExists,
    InternalServerError,
    WrongPassword,
}
