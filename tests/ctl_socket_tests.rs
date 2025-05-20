use hypr_ipc::ctl::{CtlSocket, command::Info};

#[test]
fn test_connection() {
  CtlSocket::connect().expect("Failed to establish connection");
}

#[test]
fn test_send_command() {
  let mut socket = CtlSocket::connect().expect("Failed to establish connection");

  let res = socket
    .send_command(Info::version())
    .expect("Failed to send command");

  println!("res: {:?}", res);
}
