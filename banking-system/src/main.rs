#[derive(Debug)]
struct Account{
    id : i32,
    balance : i32,
    holder : String
}

#[derive(Debug)]
struct Bank{
    account: Vec<Account>
}

impl Account{
    fn new() -> Self{
        
    }
}


fn main() {
    println!("Hello, world!");
}
