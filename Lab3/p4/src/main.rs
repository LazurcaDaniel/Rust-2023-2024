#[derive(Debug)]
#[derive(PartialEq)]
enum Error
{
    NotAscii,
    NotDigit,
    NotBase16,
    NotLetter,
    NotPrintable,
}

fn to_uppercase(c: char) -> Result<char, Error>
{
    if (c < 'a' as char || c > 'z' as char) && (c < 'A' as char || c > 'Z' as char) 
    {
        return Err(Error::NotLetter);
        
    }
    if c >= 'a' as char && c <= 'z' as char
        {
            Ok((c as u8 - 32)as char)
        }
        else {
            Ok(c)
        }
    
}

fn to_lowercase(c: char) -> Result<char, Error>
{
    if (c < 'a' as char && c > 'z' as char) || (c < 'A' as char && c > 'Z' as char) 
    {
        return Err(Error::NotLetter);
        
    }
    if c >= 'A' as char && c <= 'Z' as char
        {
            Ok((c as u8 + 32)as char)
        }
        else {
            Ok(c)
        }
    
}

fn print_char(c: char) -> Result<char, Error>
{
    if !c.is_ascii() || !c.is_ascii_graphic()
    {
        return Err(Error::NotPrintable);
    }
    Ok(c)
}

fn char_to_number(c: char) ->Result<u32, Error>
{
    if !c.is_ascii()
    {
        return Err(Error::NotAscii);
    }

    if c < '0' || c > '9'
    {
        return Err(Error::NotDigit);
    }
    Ok(c as u32 - 48)
    
}

fn char_to_number_hex(c: char) -> Result<u32,Error>
{
    if !c.is_ascii()
    {
        return Err(Error::NotAscii);
    }

    if (c < '0' || c > '9') && (c < 'A' || c > 'F')
    {   
        return Err(Error::NotBase16);
    }

    if c >= '0' && c<='9'
    {
        Ok(c as u32 - 48)
    }
    else {
        Ok(c as u32 - 55)
    }

    
}

fn print_error(e: Error)
{
    match e
    {
        Error::NotAscii =>  println!("The character that was inputed is not an ASCII character!"),
        Error::NotBase16 => println!("The character that was inputed is not in base 16!"),
        Error::NotDigit =>  println!("The character that was inputed is not a digit!"),
        Error::NotLetter => println!("The character that was inputed is not a letter!"),
        Error::NotPrintable => println!("The character that was inputed is not printable!"),
        
    }
}
fn main() {
    let upper_test = 'b';
    println!("{}",to_uppercase(upper_test).unwrap());
    let upper_test_error = to_uppercase('?');
    print_error(upper_test_error.err().unwrap());

    let lower_test = 'B';
    println!("{}",to_lowercase(lower_test).unwrap());
    let lower_test_error = to_uppercase('3');
    print_error(lower_test_error.err().unwrap());

    let printable_test = 'V';
    println!("{}",print_char(printable_test).unwrap());
    let printable_test_error = print_char('💚');
    print_error(printable_test_error.err().unwrap());

    let number_test = '5';
    println!("{}",char_to_number(number_test).unwrap());
    let number_test_error = char_to_number('u');
    print_error(number_test_error.err().unwrap());

    let base16_test = 'D';
    println!("{}",char_to_number_hex(base16_test).unwrap());
    let base16_test_error = char_to_number_hex('K');
    print_error(base16_test_error.err().unwrap());
    
    

}
