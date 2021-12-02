use std::{error::Error, env, fs::File, io::Read};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1])?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let (mut depth_naive, mut position, mut aim, mut depth_aimed) = (0i64, 0i64, 0i64, 0i64);
    for command in s.lines() {
        let parts = command.split_once(" ");
        if parts.is_none() {continue};
        let parts = parts.unwrap();
        let x = parts.1.parse::<i64>()?;
        match (parts.0.eq("forward"), parts.0.eq("down"), parts.0.eq("up")) {
            (true, false, false) => {
                position += x;
                depth_aimed += x * aim;
            },
            (false, true, false) => {
                depth_naive += x;
                aim += x;
            },
            (false, false, true) => {
                depth_naive -= x;
                aim -= x;
            },
            (_, _, _) => {println!("Unknown command: {}", parts.0);}
        }
    }
    println!("{}", depth_naive*position);
    println!("{}", depth_aimed*position);
    Ok(())
}
