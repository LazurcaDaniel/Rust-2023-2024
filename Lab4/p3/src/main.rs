use std::error::Error;
use std::time::Instant;

use std::{io, fs};
#[derive(Debug)]
enum MyError
{
    IoRead(io::Error),
}

fn change_v(v : &str) -> &str
{
    if v == "pt" || v == "ptr"
    {
        return "pentru";
    }
    if v == "dl"
    {
        return "domnul";
    }
    if v == "dna"
    {
        return "doamna";
    }
    v
}

fn p3() -> Result<(), MyError>
{
    
    let ok = fs::read_to_string("src/input.txt");

    let s = match ok
    {
        Ok(ok) => String::from(ok),
        Err(e) => return Err(MyError::IoRead(e)),
    };
    let mut abv = String::from("");
    for v in s.split(" ")
    {
        let new_v = change_v(v);
        abv.push_str(new_v);
        abv.push(' ');
    }
    println!("{:?}",abv);
    Ok(())
}

fn p()
{
    let _ok = match p3()
    {
        Ok(_ok) => {},
        Err(e) => println!("{:?}",e)
    };
}
fn main() {
    let start = Instant::now();
    p();
    println!("{:?}",start.elapsed());
    
}
