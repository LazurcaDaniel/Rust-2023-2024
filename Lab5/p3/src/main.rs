use std::fs;
use serde_derive::Deserialize;

use anyhow::Error;
use anyhow::anyhow;

#[derive(Debug, Deserialize, Copy, Clone)]
struct Student<'a> {
    name : &'a str,
    phone : &'a str,
    age : u16
}

fn main() -> Result<(), Error>{
    let content = fs::read_to_string("src/students.jsonl").unwrap();
    if content.is_empty()
    {
        return Err(anyhow!("The file is empty"));
    }
    let mut max_student = Student{name : "", phone : "", age : 0};
    let mut min_student = Student{name : "", phone : "", age : 65535};

    for i in content.split('\n'){
        let p: Student = serde_json::from_str(&i).unwrap();
        if max_student.age < p.age
        {
            max_student = p;
        }
        if min_student.age > p.age
        {
            min_student = p;
        }
    }
    print!("The oldest students is {}, aged {}, and the youngest one is {}, aged {}", max_student.name, max_student.age, min_student.name, min_student.age);
    Ok(())
}