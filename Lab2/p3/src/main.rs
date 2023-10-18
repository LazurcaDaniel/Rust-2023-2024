fn add_space(s: &mut String, n: u16) {
    let mut i = 0;
    while i < n {
        s.push(' ');
        i += 1;
    }
}

fn add_str(s: &mut String, string: &str) {
    s.push_str(&string);
}

fn recursiv(s: &mut String, integer: i64, counter : u8) {
    if integer != 0 {
        
        let x: char = ((integer % 10) as u8 + '0' as u8) as char;
        recursiv(s, integer / 10, counter+1);
        
        s.push(x);
        if counter % 3 == 0 && counter != 0
        {
            s.push('_');
        }
    }
}

fn add_integer(s: &mut String,  integer: i64) {
    if integer == 0
    {
        s.push('0');
    }
    else {
        recursiv(s, integer, 0);
    }
}

fn count_digits(mut x : i64) -> i8
{
    let mut nr = 0;
    if x == 0
    {
        return 1;
    }
    while x > 0
    {
        nr += 1;
        x /=10;
    }
    nr as i8
}

fn recursive_float(s: &mut String, float: i64, digits : i8)
{
    if float != 0
    {
        let x: char = ((float % 10) as u8 + '0' as u8) as char;
        recursive_float(s, float/10, digits-1);
        if digits == 1
        {
            s.push('.');
        }
        s.push(x);
        
    }
}


fn add_float(s: &mut String, mut float: f32) {
    if float == 0.0
    {
        s.push_str("0.0");
    }
    else {
        let int_part : i64 = float as i64;
        while float != (float as i64) as f32
        {
            float *= 10.0;
        }

        let digits_before = count_digits(int_part);
        let digits_after = count_digits(float as i64) - digits_before;
        recursive_float(s,float as i64, digits_after);
    }
}

fn main() {
    let mut s : String = String::from("");
    add_space(&mut s, 40);
    add_str(&mut s, "I");
    add_space(&mut s, 1);
    add_str(&mut s, "💚\n");
    add_space(&mut s, 40);
    add_str(&mut s, "RUST.\n\n");
    add_space(&mut s, 4);
    add_str(&mut s, "Most");
    add_space(&mut s, 12);
    add_str(&mut s, "crate");
    add_space(&mut s, 6);
    add_integer(&mut s, 306437968);
    add_space(&mut s, 11);
    add_str(&mut s, "and");
    add_space(&mut s, 5);
    add_str(&mut s, "lastest");
    add_space(&mut s, 8);
    add_str(&mut s, "is\n");
    add_space(&mut s, 9);
    add_str(&mut s, "downloaded");
    add_space(&mut s, 8);
    add_str(&mut s, "has");
    add_space(&mut s, 13);
    add_str(&mut s, "downloads");
    add_space(&mut s, 5);
    add_str(&mut s, "the");
    add_space(&mut s, 9);
    add_str(&mut s, "version");
    add_space(&mut s, 4);
    add_float(&mut s, 2.038);
    add_str(&mut s, ".");
    println!("{s}");
}
