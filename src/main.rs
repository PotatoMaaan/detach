use std::{
    env,
    process::{exit, Command, Stdio},
};

fn main() {
    let user_input = env::args_os().skip(1).collect::<Vec<_>>();
    let Some((program, args)) = user_input.split_first() else {
        eprintln!("No input provided");
        return;
    };

    Command::new(&program)
        .args(args)
        .stderr(Stdio::null())
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .spawn()
        .unwrap_or_else(|e| {
            eprintln!("Failed to spawn child process: {}", e);
            exit(1);
        });
}
