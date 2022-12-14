use crate::errors::Errcode;
use crate::ipcs::generate_socketpair;

use std::os::unix::io::RawFd;

use std::ffi::CString;
use std::path::PathBuf;

#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,
    pub fd: RawFd,
    pub uid: u32,
    pub mount_dir: PathBuf,
}

impl ContainerOpts {
    pub fn new(command: String, uid: u32, mount_dir: PathBuf) -> Result<(ContainerOpts, (RawFd, RawFd)), Errcode> {
        let sockets = generate_socketpair()?;

        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read arg"))
            .collect();
        let path = argv[0].clone();

        Ok((ContainerOpts {
            path,
            argv,
            fd: sockets.1.clone(),
            uid,
            mount_dir,
        }, sockets))
    }
}
