
fn prime(x: u16) -> bool
{
    if x < 2 
    {
        return false;
    }
    if x == 2
    {
        return true;
    }
    if x%2 == 0
    {
        return false;
    }
    let mut i = 3;
    while i <= x/2
    {
        if x%i == 0 
        {
            return false;
        }
        i = i+2;
    }
    return true;
}

fn next_prime(x: u16) -> Option<u16>
{
    let mut i = x+1;
    while i < 65534
    {
        if prime(i)
        {
            return Some(i);
        }
        i = i+1;
    }
    return None;
}
fn main() {
    let mut x : u16 = 6426;
    while let Some(i) = next_prime(x) {
        print!("{i} ");
        x = i;
    }
}

