use std::{fs, io};


fn p1() -> Result<(), io::Error>
{
    let s = fs::read_to_string("src/text.txt")?;
    

    let mut max_length = 0;
    let mut max_length_string = String::from("");

    let mut max_bytes = 0;
    let mut max_bytes_string = String::from("");

    for v in s.split("\n")
    {
        let mut current_length = 0;
        
        for _i in v.chars(){
            current_length+=1;
        }
        if max_length < current_length
        {
            max_length = current_length;
            max_length_string = v.to_string();
        }
        
        if max_bytes < v.len()
        {
            max_bytes = v.len();
            max_bytes_string = v.to_string();
        }
    }
    println!("The longest string in number of chars is {}",max_length_string);
    println!("The longest string in bytes is {}",max_bytes_string);

    Ok(())
    
}


fn main() {
    
    let str = p1();
    match str
    {
        Ok(str) => {}
        Err(e) => println!("{}",e),
    }


}
