use std::{
  io::{Read, Write},
  os::unix::net::UnixStream,
  path::PathBuf,
};

use command::Command;

use crate::{
  env::{his, xdg_runtime_dir},
  error::{self, Error},
};

pub mod command;
pub mod info;

pub struct CtlSocket {
  socket: UnixStream,
}

impl CtlSocket {
  pub fn connect() -> error::Result<Self> {
    let his = his()?;
    let xdg_runtime_dir = xdg_runtime_dir()?;
    let path = PathBuf::from(xdg_runtime_dir)
      .join("hypr")
      .join(his)
      .join(".socket.sock");
    let socket = UnixStream::connect(path).map_err(|_| Error::ConnectionError)?;
    Ok(Self { socket })
  }

  pub fn send_command<C>(&mut self, command: C) -> error::Result<C::Response>
  where
    C: Command,
  {
    let _ = command.write_command(&mut self.socket)?;
    let res = command.read_response(&mut self.socket)?;
    Ok(res)
  }
}
