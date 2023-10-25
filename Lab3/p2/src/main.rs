
fn checked_addition(x : u32, y: u32) -> u32 
{
    if x as u64 + y as u64 > std::u32::MAX as u64
    {
        panic!("Addition: Overflow!");
    }
    else {
        x+y
    }

}

fn checked_multiplication(x: u32, y: u32) -> u32
{
    if x as u64 * y as u64 > std::u32::MAX as u64
    {
        panic!("Multiplication: Overflow!");
    }
    else {
        x*y
    }
}

fn main() {
    let r = checked_addition(3000000, 1999999999);
    print!("{r}");
    let b  = checked_multiplication(3000000000, 1999999999);
    println!("{b}");
}
