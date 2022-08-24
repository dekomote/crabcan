use log;
use std::fmt;
use std::process::exit;

#[derive(Debug)]
// Contains all possible errors in our tool
pub enum Errcode {
    InvalidArgument(&'static str),
    UnsuportedVersion(u8),
    ContainerError(u8),
    NotSupported(u8)
}

#[allow(unreachable_patterns)]
impl fmt::Display for Errcode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Errcode::InvalidArgument(arg) => write!(f, "Invalid Argument: {}", arg),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Errcode {
    pub fn get_retcode(&self) -> i32 {
        1
    }
}

// Get the result from a function and exit the process with the correct error code
pub fn exit_with_retcode(res: Result<(), Errcode>) {
    match res {
        Ok(_) => {
            log::debug!("Exit without an error code, returning 0");
            exit(0);
        }
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit\n\t{}\n\tReturning {}", e, retcode);
            exit(retcode);
        }
    }
}
