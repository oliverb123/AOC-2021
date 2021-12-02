use std::{fs::File, env, io::Read, error::Error};
use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let mut f = File::open(&args[1])?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    let result: usize = s.lines().into_iter()
        .flat_map(|s| s.parse::<usize>())
        .tuple_windows::<(_, _)>()
        .filter(|t| t.0 < t.1)
        .count();
    println!("{}", result);
    let result: usize = s.split("\n").into_iter()
        .flat_map(|s| s.parse::<usize>())
        .tuple_windows::<(_, _, _, _)>()
        .filter(|t| t.0 < t.3)
        .count();
    println!("{}", result);
    Ok(())
}
