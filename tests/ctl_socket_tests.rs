use std::fmt::Debug;

use hypr_ipc::ctl::{
  CtlSocket,
  command::{Command, Info},
};

fn connect() -> CtlSocket {
  CtlSocket::connect().expect("Failed to establish connection")
}

fn test_send_command<C, R>(command: C)
where
  C: Command<Response = R>,
  R: Debug,
{
  let mut socket = connect();

  let res = socket
    .send_command(command)
    .expect("Failed to send command");

  println!("res: {:?}", res);
}

#[test]
fn test_connection() {
  connect();
}

#[test]
fn test_version_info() {
  test_send_command(Info::version());
}

#[test]
fn test_monitors_info() {
  test_send_command(Info::monitors());
}

#[test]
fn test_workspaces_info() {
  test_send_command(Info::workspaces());
}
