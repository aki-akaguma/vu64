use anyhow::Context;

pub fn run(program: &str, args: &[&str]) -> anyhow::Result<()> {
    //println!("program: {_program}");
    //println!("args: {_args:#?}");
    if args.is_empty() {
        print_help_and_exit(program);
    }
    let opt = args[0];
    let file_path = args[1];
    #[rustfmt::skip]
    match opt {
        "-d" => decoder(file_path)?,
        "-e" => encoder(file_path)?,
        //
        "--help" | "-h" | "-H" | "help" => print_help_and_exit(program),
        "--version" | "-V" | "-v" => print_version_and_exit(program),
        _ => {
            eprintln!("Not fount option: {opt}");
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
    println!(
        "[usage] {} {{ {} }} file_path",
        program,
        concat!("-d | ", "-e")
    );
    std::process::exit(0);
}

fn decoder(file_path: &str) -> anyhow::Result<()> {
    use std::io::Read;
    //
    let f = std::fs::File::open(file_path).context(format!("open(\"{file_path}\")"))?;
    let reader = std::io::BufReader::new(f);
    let mut temp: Vec<u8> = Vec::new();
    for byt in reader.bytes() {
        let byte = byt?;
        temp.push(byte);
        if temp.len() >= 8 {
            //
            let mut s = String::new();
            for a in &temp {
                s += &format!("0x{a:02x}, ");
            }
            if s.len() > 2 {
                s = s[..(s.len() - 2)].to_string();
            }
            println!("[{s}]");
            //
            let _value = vu64::decode(&temp)?;
            temp.clear();
        }
    }
    //
    Ok(())
}

fn encoder(file_path: &str) -> anyhow::Result<()> {
    use std::io::Read;
    use vu64::io::WriteVu64;
    //
    let buf: Vec<u8> = Vec::new();
    let mut crsr = std::io::Cursor::new(buf);
    //
    let f = std::fs::File::open(file_path).context(format!("open(\"{file_path}\")"))?;
    let mut reader = std::io::BufReader::new(f);
    let mut temp = [0u8; 8];
    reader.read_exact(&mut temp)?;
    let value = u64::from_le_bytes(temp);
    crsr.encode_and_write_vu64(value)?;
    //
    Ok(())
}
