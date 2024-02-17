use std::{
    env,
    process::{Command, Stdio},
};

fn main() {
    let user_input = env::args_os().skip(1).collect::<Vec<_>>();
    let Some((program, args)) = user_input.split_first() else {
        eprintln!("No input provided");
        return;
    };

    if let Err(e) = Command::new(&program)
        .args(args)
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
    {
        eprintln!("Failed to spawn child process: {}", e);
    }
}
