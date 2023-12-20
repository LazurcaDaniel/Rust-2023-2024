use std::cell::RefCell;


const LIMIT_SIZE : usize = 10;
struct LinkedHash
{
    num : u64,
    result : bool,
    next: Option<Box<LinkedHash>>,
    size : usize,
}

struct Cache {
    cache: RefCell<LinkedHash>,
}

impl LinkedHash
{
    fn new(num : u64, result : bool) -> Self
    {
        LinkedHash{
            num,
            result,
            next : None,
            size: 1,
        }
    }
    fn append(&mut self, num:u64, result:bool)
    {
        if let Some(ref mut next) = self.next
        {
            next.append(num,result);
        }
        else {
            let new_node = Box::new(LinkedHash::new(num,result));
            self.next = Some(new_node);
            self.size+=1;
        }
    }

    fn limit_size(&mut self, limit : usize ){
        if self.size > limit
        {
            if let Some(mut next) = self.next.take()
            {
                self.next = next.next.take();
                self.size -= 1;
            }
        }
        if let Some(ref mut next) = self.next
        {
            next.limit_size(limit);
        }
    }   
        fn get_result(&self, num: u64) -> Option<bool> {
            if self.num == num {
                Some(self.result)
            } else if let Some(ref next) = self.next {
                next.get_result(num)
            } else {
                None
            }
        }
    
}

impl Cache
{
    fn new(num: u64, resut:bool) -> Self
    {
        Cache{
            cache : RefCell::new(LinkedHash::new(num,resut)),
        }
    }
    fn is_prime_cached(&self, num: u64) -> Option<bool> {
        self.cache.borrow().get_result(num)
    }
    fn insert_cache(&self, num: u64, result: bool) {
        let mut borrow = self.cache.borrow_mut();
        borrow.append(num, result);
        borrow.limit_size(LIMIT_SIZE);
    }

    fn is_prime(&self, num: u64) -> bool
    {
        if let Some(result) = self.is_prime_cached(num){
            println!("Got it from cache!");
            result
        }
        else {
            let result = is_nr_prime(num);
            self.insert_cache(num, result);
            result
        }
    }
}

fn is_nr_prime(x : u64) -> bool
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
    let mut d = 3;
    while d*d <= x
    {
        if x%d == 0
        {
            return false;
        }
        d = d+2;
    }
    true
}
fn main() {
    
    let cache  = Cache::new(0, false);
    loop
    {
        println!("Enter a number!");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        let num: u64 = input.trim().parse().expect("Invalid input");
        let result = cache.is_prime(num);
        
        if result
        {
            println!("{} is prime!", num);
        }
        else {
            println!("{} is not prime!",num);
        }
    }
}