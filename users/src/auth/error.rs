#[derive(Debug)]
pub enum CreateUserError {
    UserAlreadyExists,
    DatabaseError,
    HashPasswordError,
}

#[derive(Debug)]
pub enum CheckPasswordError {
    WrongPassword,
    DatabaseError,
}
