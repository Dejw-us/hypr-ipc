use std::{
  io::{Read, Write},
  os::unix::net::UnixStream,
  path::PathBuf,
};

use hypr_ipc::{ctl::info::Version, env};

#[test]
fn test_socket() {
  let his = env::his().expect("Ho HIS");
  let xdg_runtime_dir = env::xdg_runtime_dir().expect("No xdg runtime dir");

  let path = PathBuf::from(xdg_runtime_dir)
    .join("hypr")
    .join(his)
    .join(".socket.sock");

  println!("Connecting to {:?}", path);

  let mut stream = UnixStream::connect(path).expect("Failed to connect");
  let command = "j/version";

  stream
    .write_all(command.as_bytes())
    .expect("Failed to write data");

  let mut res = String::new();

  stream
    .read_to_string(&mut res)
    .expect("Failed to read response");

  let version: Version = serde_json::from_str(&res).expect("Failed to deserialize");

  println!("Response: {:?}", version);
}
