use std::{io, fs};
use std::time::Instant;
#[derive(Debug)]
enum MyError
{
    IoRead(io::Error),
}

fn p4() -> Result<(), MyError>
{
    let ok = fs::read_to_string("C:\\Windows\\System32\\drivers\\etc\\hosts");
    let s = match ok
    {
        Ok(ok) => String::from(ok),
        Err(e) => return Err(MyError::IoRead(e)),
    };

    for v in s.split('\n')
    {
        if !v.starts_with("#") && !v.starts_with(" ")
        {
            let mut f_part = String::from("");
            let mut s_part = String::from("");
            let mut i = 0;
            for v1 in v.splitn(2," ")
            {
                i+=1;
                if i == 1
                {
                    f_part = v1.to_string();
                }
                if i == 2
                {
                    s_part = v1.to_string();
                    break;
                }
            }
            println!("{} => {}",s_part,f_part);
        }
    }
    Ok(())
}

fn p()
{
    match p4()
    {
        Ok(_ok) => {},
        Err(e) => println!("{:?}",e),
    }
}

fn main() {
    let start = Instant::now();
    p();
    println!("{:?}", start.elapsed());
    
}