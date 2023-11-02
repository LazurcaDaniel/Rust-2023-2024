use std::time::Instant;

use std::{io, fs};
#[derive(Debug)]
enum MyError
{
    NotAscii,
    IoRead(io::Error),
    IoWrite(io::Error),
}

fn rot_char(c : char) -> char
{
    if (c >= 'a' && c <= 'm') || (c >= 'A' && c <= 'M') 
        {
            (c as u8 + 13) as char
        }
    else if (c >='n' && c<='z') || (c >= 'N' && c <= 'Z')
    {
        (c as u8 - 13) as char
    }
    else 
    {
        c
    }
}

fn p2() -> Result<(),MyError>
{
    let ok = fs::read_to_string("src/input.txt");

    let s = match ok
    {
        Ok(ok) => String::from(ok),
        Err(e) => return Err(MyError::IoRead(e)),
    };
    let mut rot13 = String::from("");

    for i in s.chars()
    {
        if !i.is_ascii()
        {
            return Err(MyError::NotAscii);
        }
        rot13.push(rot_char(i));
    }

    match fs::write("src/output.txt", &rot13)
    {
        Ok(_ok) =>{},
        Err(e) => return Err(MyError::IoWrite(e)),

    }
    Ok(())
}
fn p()
{
    let _ok =
    match p2()
    {
        Ok(_ok) => {},
        Err(e) => println!("Eroare! {:?}",e),
    };
}
fn main() {
    let start = Instant::now();
    p();
    println!("{:?}",start.elapsed());
    
}
