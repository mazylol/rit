use std::env;
use std::process::Command;

fn main() {
  let args: Vec<String> = env::args().collect();
  let cmds = &args[1];

  if cmds == "status" {
    status();
  } else if cmds == "commit" {
    commit();
  } else if cmds == "add" {
    add();
  } else {
    println!("Invalid command");
  }
}

fn status() {
  let cmd = Command::new("/bin/git").arg("status").output().expect("failed to execute process");
  println!("{}", String::from_utf8_lossy(&cmd.stdout));
  println!("{}", String::from_utf8_lossy(&cmd.stderr));
}

fn commit() {
  let args: Vec<String> = env::args().collect();
  let cmd = Command::new("/bin/git")
    .arg("commit")
    .arg("-m")
    .arg(&args[0])
    .output()
    .expect("failed to execute process");
  println!("{}", String::from_utf8_lossy(&cmd.stdout));
  println!("{}", String::from_utf8_lossy(&cmd.stderr));
}

fn add() {
  let args: Vec<String> = env::args().collect();
  let cmd =
    Command::new("/bin/git").arg("add").arg(&args[2]).output().expect("failed to execute process");
  println!("status: {}", &cmd.status);
  println!("{}", String::from_utf8_lossy(&cmd.stdout));
  println!("{}", String::from_utf8_lossy(&cmd.stderr));
}
