use std::env;
use std::process::exit;
use std::process::Command;

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 2 {
    println!("Usage: {} <command>", args[0]);
    exit(1);
  }

  let cmds: &String = &args[1];

  if cmds == "" {
    println!("Enter a command to run");
  } else if cmds == "status" {
    status();
  } else if cmds == "commit" {
    commit();
  } else if cmds == "add" {
    add();
  } else if cmds == "reset" {
    reset();
  } else if cmds == "push" {
    push();
  } else if cmds == "doitall" {
    doitall();
  } else if cmds == "diff" {
    diff();
  } else if cmds == "init" {
    init();
  } else {
    println!("Invalid command!");
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
    .arg(&args[2])
    .output()
    .expect("failed to execute process");
  println!("{}", String::from_utf8_lossy(&cmd.stdout));
  println!("{}", String::from_utf8_lossy(&cmd.stderr));
}

fn add() {
  let args: Vec<String> = env::args().collect();
  let cmd =
    Command::new("/bin/git").arg("add").arg(&args[2]).output().expect("failed to execute process");
  println!("{}", String::from_utf8_lossy(&cmd.stdout));
  println!("{}", String::from_utf8_lossy(&cmd.stderr));
}

fn reset() {
  let cmd = Command::new("/bin/git")
    .arg("reset")
    .arg("--hard")
    .arg("HEAD~1")
    .output()
    .expect("failed to execute process");
  println!("{}", String::from_utf8_lossy(&cmd.stdout));
  println!("{}", String::from_utf8_lossy(&cmd.stderr));
}

fn push() {
  let cmd = Command::new("/bin/git")
    .arg("push")
    .arg("origin")
    .arg("master")
    .output()
    .expect("failed to execute process");
  println!("{}", String::from_utf8_lossy(&cmd.stdout));
  println!("{}", String::from_utf8_lossy(&cmd.stderr));
}

fn doitall() {
  let cmd =
    Command::new("/bin/git").arg("add").arg("-A").output().expect("failed to execute process");
  println!("{}", String::from_utf8_lossy(&cmd.stdout));
  println!("{}", String::from_utf8_lossy(&cmd.stderr));
  commit();
  push();
}

fn diff() {
  let cmd = Command::new("/bin/git").arg("diff").output().expect("failed to execute process");
  println!("{}", String::from_utf8_lossy(&cmd.stdout));
  println!("{}", String::from_utf8_lossy(&cmd.stderr));
}

fn init() {
  let args: Vec<String> = env::args().collect();
  let cmd =
    Command::new("/bin/git").arg("init").arg(&args[2]).output().expect("failed to execute process");
  println!("{}", String::from_utf8_lossy(&cmd.stdout));
  println!("{}", String::from_utf8_lossy(&cmd.stderr));
}
