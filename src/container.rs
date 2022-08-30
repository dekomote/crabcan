use nix::sys::utsname::uname;
use nix::sys::wait::waitpid;

use crate::{cli::Args, child::generate_child_process};
use crate::config::ContainerOpts;
use crate::errors::Errcode;
use nix::unistd::{close, Pid};
use scan_fmt::scan_fmt;
use std::os::unix::io::RawFd;

pub const MINIMAL_KERNEL_VERSION: f32 = 4.8;

pub struct Container {
    sockets: (RawFd, RawFd),
    config: ContainerOpts,
    child_pid: Option<Pid>
}

impl Container {
    pub fn new(args: Args) -> Result<Container, Errcode> {
        let (config, sockets) = ContainerOpts::new(args.command, args.uid, args.mount_point)?;
        Ok(Container { sockets, config, child_pid: None})
    }

    pub fn create(&mut self) -> Result<(), Errcode> {
        let pid = generate_child_process(self.config.clone())?;
        self.child_pid = Some(pid);
        log::debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), Errcode> {
        if let Err(e) = close(self.sockets.0) {
            log::error!("Unable to close write socket: {:?}", e);
            return Err(Errcode::SocketError(3));
        }
        if let Err(e) = close(self.sockets.1) {
            log::error!("Unable to close read socket: {:?}", e);
            return Err(Errcode::SocketError(3));
        }
        log::debug!("Cleaning container");
        Ok(())
    }
}

pub fn start(args: Args) -> Result<(), Errcode> {
    check_linux_version()?;
    let mut container = Container::new(args)?;
    if let Err(e) = container.create() {
        container.clean_exit()?;
        log::error!("Error while creating container: {:?}", e);
        return Err(e);
    }
    log::debug!("Container with child PID: {:?}", container.child_pid);
    wait_child(container.child_pid);
    log::debug!("Finished, cleaning & exiting");
    container.clean_exit()
}

pub fn check_linux_version() -> Result<(), Errcode> {
    let host = uname().unwrap();
    let release_str: String = host.release().to_str().unwrap().into();
    log::debug!("Linux release: {}", &release_str);

    if let Ok(version) = scan_fmt!(&release_str, "{f}.{}", f32) {
        if version < MINIMAL_KERNEL_VERSION {
            return Err(Errcode::UnsuportedVersion(0));
        }
    } else {
        return Err(Errcode::ContainerError(0));
    }

    Ok(())
}

pub fn wait_child(pid: Option<Pid>) -> Result<(), Errcode>{
    if let Some(child_pid) = pid {
        log::debug!("Waiting for child (pid {}) to finish", child_pid);
        if let Err(e) = waitpid(child_pid, None){
            log::error!("Error while waiting for pid to finish: {:?}", e);
            return Err(Errcode::ContainerError(1));
        }
    }
    Ok(())
}
