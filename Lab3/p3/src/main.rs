
#[derive(Debug)]
enum Error
{
    Overflow,
}

fn checked_addition(x : u32, y: u32) -> Result<u32, Error>
{
    let sum = x as u64 + y as u64;
    if sum > std::u32::MAX as u64
    {
        return Err(Error::Overflow);
    }
    return Ok(sum as u32);
}

fn checked_multiplication(x: u32, y: u32) -> Result<u32, Error>
{
    let product = x as u64 * y as u64;
    if product > std::u32::MAX as u64
    {
        return Err(Error::Overflow);
    }
    return Ok(product as u32);
}

fn propagate_errors(x: u32, y:u32, op:char) -> ()
{
    if op == '+'
    {
        let r = checked_addition(x,y);
        if r.is_err()
        {
            println!("Error found: {:?}", r);
        }
        else {
            println!("Addition succsesfull! The result is: {}", r.ok().unwrap());
        }
        
    }
    else if op == '*'{
        let b  = checked_multiplication(x,y);
        if b.is_err()
        {
            println!("Error found: {:?}", b);
        }
        else {
            println!("Multiplication succsesfull! The result is: {}", b.ok().unwrap());
        }
    }
    else {
        println!("Error: Wrong operand. Use * or +");
    }


}

fn main() {
    propagate_errors(5454, 643263, '+');
    propagate_errors(100000000, 3000000, '*');
}
