#[derive(Debug)]
enum Error
{
    NotInteger,
    NegativeNumber,
}

fn is_prime(x: f64) -> Result<bool, Error>
{
    if x.fract() != 0.0
    {
        return Err(Error::NotInteger);
    }
    if x < 0.0
    {
        return Err(Error::NegativeNumber);
    }
    let y = x as u32;
    if y < 2
    {
        return Ok(false);
    }
    if y == 2
    {
        return Ok(true);
    }
    if y % 2 == 0
    {
        return Ok(false);
    }
    let mut i = 3;
    while i*i < y
    {
        if y%i == 0
        {
            return Ok(false);
        }
        i = i+2;
    }
    Ok(true)
}

fn propagate_error(e:Error) -> ()
{
    match e
    {
        Error::NegativeNumber => println!("The number is negative!"),
        Error::NotInteger => println!("The number is not an integer!"),
    }
}

fn main() {
    let test1  = is_prime(13.0);
    let test2= is_prime(13.6);
    let test3 = is_prime(-16.0);
    if test1.is_ok()
    {
        println!("{}",test1.ok().unwrap());
    }
    else {
        propagate_error(test1.err().unwrap());
    }
    if test2.is_ok()
    {
        println!("{}",test2.ok().unwrap());
    }
    else {
        propagate_error(test2.err().unwrap());
    }
    if test3.is_ok()
    {
        println!("{}",test3.ok().unwrap());
    }
    else {
        propagate_error(test3.err().unwrap());
    }
}
