use std::collections::HashMap;
use std::fs;

use anyhow::anyhow;
use anyhow::Result;
fn solve() -> Result<()> {
    if let Ok(s) = fs::read_to_string("src/input.txt") {
        let mut map = HashMap::<String, u32>::new();
        let mut allign = 0;
        for i in s.split(|c: char| !c.is_alphanumeric()) {
            if i == "" {
                continue;
            }
            if s.len() > allign {
                allign = i.len();
            }
            map.entry(i.to_string().to_lowercase())
                .and_modify(|x| *x += 1)
                .or_insert(1);
        }
        let mut v: Vec<_> = map.into_iter().collect();
        v.sort_unstable_by_key(|x| -(x.1 as i32));
        for (key, value) in v.iter() {
            println!("{0:<1$} => {2}", key, allign, value);
        }
        Ok(())
    } else {
        return Err(anyhow!("Error reading from the file!"));
    }
}

fn main() -> Result<()> {
    match solve() {
        Ok(_) => {
            println!("Execution successful!");
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }

    Ok(())
}
