use std::{env, fs::File, io::Read, mem};
use std::error::Error;
use regex::Regex;

mod events;

use crate::events::*;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("USAGE: {} devpath", args[0]);
        return Ok(());
    }

    let devpath_re = Regex::new(r"^/dev/input/event[0-9]+$")?;
    let devpath = &args[1];

    if !devpath_re.is_match(devpath) {
        eprintln!("Devpath must be an absolute path to the character device (e.g. /dev/input/event0)");
        return Ok(());
    }

    let mut f = File::open(devpath)?;
    let mut buffer: Vec<u8> = vec![0; mem::size_of::<Input>()];
    let mut event = Input::default();

    loop {
        let _ = f.read_exact(&mut buffer)?;
        let _ = event.copy_from_bytes(&buffer)?;
        println!("{event}");
    }
}
