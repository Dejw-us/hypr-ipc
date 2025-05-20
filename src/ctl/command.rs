use std::{
  io::{self, Read, Write},
  marker::PhantomData,
  os::unix::net::UnixStream,
};

use crate::error::{self, Error};

use super::info::Version;

fn read_to_string(socket: &mut UnixStream) -> error::Result<String> {
  let mut buf = String::new();
  socket
    .read_to_string(&mut buf)
    .map_err(|_| Error::SocketReadError)?;
  Ok(buf)
}

fn write_all(socket: &mut UnixStream, command: &[u8]) -> error::Result<()> {
  socket
    .write_all(command)
    .map_err(|_| Error::SocketWriteError)
}

pub trait Command {
  type Response;

  fn write_command(&self, socket: &mut UnixStream) -> error::Result<()>;

  fn read_response(&self, socket: &mut UnixStream) -> error::Result<Self::Response>;
}

impl Command for &str {
  type Response = String;

  fn write_command(&self, socket: &mut UnixStream) -> error::Result<()> {
    write_all(socket, self.as_bytes())
  }

  fn read_response(&self, socket: &mut UnixStream) -> error::Result<Self::Response> {
    read_to_string(socket)
  }
}

pub struct Info<T = Version> {
  phantom: PhantomData<T>,
}

impl Info {
  pub fn version() -> Info<Version> {
    Info {
      phantom: PhantomData,
    }
  }
}

impl Command for Info<Version> {
  type Response = Version;

  fn write_command(&self, socket: &mut UnixStream) -> error::Result<()> {
    write_all(socket, "j/version".as_bytes())
  }

  fn read_response(&self, socket: &mut UnixStream) -> error::Result<Self::Response> {
    let data = read_to_string(socket)?;
    let version: Version = serde_json::from_str(&data)?;
    Ok(version)
  }
}
