use std::{
  io::{self, Read, Write},
  marker::PhantomData,
  os::unix::net::UnixStream,
};

use serde::{Deserialize, de::DeserializeOwned};

use crate::error::{self, Error};

use super::info::{Monitors, Version, Workspaces};

macro_rules! impl_json_response_command {
  ($response_type:ty, $command:expr) => {
    impl Command for Info<$response_type> {
      type Response = $response_type;

      fn write_command(&self, socket: &mut UnixStream) -> error::Result<()> {
        write_all(socket, $command.as_bytes())
      }

      fn read_response(&self, socket: &mut UnixStream) -> error::Result<Self::Response> {
        read_to_json(socket)
      }
    }
  };
}

fn read_to_string(socket: &mut UnixStream) -> error::Result<String> {
  let mut buf = String::new();
  socket
    .read_to_string(&mut buf)
    .map_err(|_| Error::SocketReadError)?;
  Ok(buf)
}

fn read_to_json<T>(socket: &mut UnixStream) -> error::Result<T>
where
  T: DeserializeOwned,
{
  let mut buf = String::new();
  socket
    .read_to_string(&mut buf)
    .map_err(|_| Error::SocketReadError)?;
  let res: T = serde_json::from_str(&buf)?;
  Ok(res)
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

  pub fn monitors() -> Info<Monitors> {
    Info {
      phantom: PhantomData,
    }
  }

  pub fn workspaces() -> Info<Workspaces> {
    Info {
      phantom: PhantomData,
    }
  }
}

impl_json_response_command!(Version, "j/version");
impl_json_response_command!(Monitors, "j/monitors");
impl_json_response_command!(Workspaces, "j/workspaces");
