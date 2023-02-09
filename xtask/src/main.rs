//
// ref)
//   https://github.com/matklad/cargo-xtask
//
mod tester;

fn main() -> anyhow::Result<()> {
    let mut env_args: Vec<String> = std::env::args().collect();
    let program = env_args.remove(0);
    if env_args.is_empty() {
        print_help_and_exit(&program);
    }
    let cmd = env_args[0].as_str();
    let program = &program;
    let env_args: Vec<&str> = env_args[1..].iter().map(|s| s.as_str()).collect();
    #[rustfmt::skip]
    match cmd {
        "tester" => tester::run(&format!("{} {}", program, cmd), &env_args)?,
        //
        "--help" | "-h" | "-H" | "help" => print_help_and_exit(program),
        "--version" | "-V" | "-v" => print_version_and_exit(program),
        _ => {
            eprintln!("Not fount command: {}", cmd);
            unreachable!()
        }
    };
    //
    Ok(())
}

fn print_version_and_exit(_program: &str) {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    std::process::exit(0);
}

fn print_help_and_exit(program: &str) {
    println!("[usage] {} {{ {} }}", program, concat!("tester",));
    std::process::exit(0);
}
