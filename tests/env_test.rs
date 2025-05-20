use hypr_ipc::env;

#[test]
fn env_test() {
  let his = env::his().expect("Ho HIS");
  let xdg_runtime_dir = env::xdg_runtime_dir().expect("No xdg runtime dir");

  println!("HIS: {}, xdg runtime dir: {}", his, xdg_runtime_dir);
}
