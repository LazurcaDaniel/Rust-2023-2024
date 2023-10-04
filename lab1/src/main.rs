fn prime(x: i32) -> bool {
    if x < 2 {
        return false; 
    }
    if x == 2 {
        return true;
    }
    if x % 2 == 0 {
        return false;
    }
    let mut d = 3;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 2;
    }
    return true;
}

fn cmmdc(mut a : i32, mut b:i32) -> i32
{
    while b > 1
    {
        let r = a%b;
        a = b;
        b = r;
    }
    b 
}

fn p1()
{
    let mut i = 2;
    while i < 100 {
        if prime(i) {
            print!("{i} ");
        }
        i = i + 1;
    }
}

fn p2()
{
    let mut a = 1;
    while a <= 100
    {
        let mut b = a+1;
        while b<=100
        {
            if cmmdc(a,b) == 1
            {
                print!("({a},{b}) ");
            }
            b +=1;
        }
        a+=1;
    }
}

fn p3()
{
    let mut i = 99;
    while i>=1 
    {
        println!("{i} bottles of beer on the wall, ");
        println!("{i} bottles of beer, ");
        println!("Take one down, pass it around, ");
        i -=1;
        if i>0 
        {
            println!("{i} bottles of beer on the wall.\n");
        }
        else {
            println!("No bottles of beer on the wall.\n");
        }
        }
}

fn main() {
    // Problema 1
    p1();
    println!("\n");
    //Problema 2
    p2();
    println!("\n");
    // Problema 3
    p3();
}

