use std::fs;

use anyhow::Result;
use anyhow::anyhow;
#[derive(Debug,Clone, Copy)]
struct Student<'a>
{
    name : &'a str,
    phone_number: &'a str, 
    age : i32
}

fn string_to_number(x : &str) -> i32
{
    let mut o = 0;
    for i in x.to_string().chars()
    {
        if i >= '0' && i<='9'{
        o = o*10 + (i as u8 - '0' as u8) as i32;
        }
    }
    o
}

fn p1() -> Result<()>
{
    let s = fs::read_to_string("src/input.txt")?;
    if s.is_empty()
    {
        return Err(anyhow!("The file is empty"));
    }
    let mut students: [Student; 100] = [
        Student {
            name: "",
            phone_number: "",
            age: 0,
        };
        100
    ];
    let mut nr_students = 0;
    let mut max_student : Student = Student{name: "", phone_number : "", age : 0};
    let mut min_student : Student = Student{name: "", phone_number : "", age : 9999999};
    for v in s.split("\n")
    {
        if nr_students > 100
        {
            return Err(anyhow!("Too many students!"));
        }
        let mut field = 0;
        for i in v.split(",")
        {
            
            if field == 0
            {
                students[nr_students].name = i; 
            }
            else if field == 1
            {
                students[nr_students].phone_number = i;
            }
            else {
                students[nr_students].age = string_to_number(i);
                if students[nr_students].age > max_student.age
                {
                    max_student = students[nr_students];
                }
                if students[nr_students].age < min_student.age
                {
                    min_student = students[nr_students];
                }
            }
            field +=1;
        }
        nr_students += 1;
    }
    println!("The oldest student is {}, aged {}, and the youngest one is {}, aged {}", max_student.name, max_student.age, min_student.name, min_student.age);
    Ok(())
}

fn main() -> Result<()> {
   let x = p1()?;
    Ok(())
}
