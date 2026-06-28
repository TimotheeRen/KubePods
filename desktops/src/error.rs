#[derive(Debug)]
pub enum ProvisioningError {
    NoDesktopFound,
    DesktopAlreadyExist,
    InternalServerError,
}
