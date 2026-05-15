#[derive(Debug)]
pub enum CreateUserError {
    UserAlreadyExists,
    DatabaseError,
}
