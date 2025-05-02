#[derive(Debug)]
struct Account{
    id : i32,
    balance : i32,
    holder : String
}

impl Account{
    fn new(id:i32, holder:String) -> Self{
        Account { id, balance: 0, holder }
    }
}

#[derive(Debug)]
struct Bank{
    accounts: Vec<Account>
}

impl Bank{
    fn new() -> Self{
        Bank { accounts: vec![] }
    }
}

fn print_account(account:&Account){
    println!("{:#?}",account);
}


fn main() {
    //let mut new_bank = Bank::new();
    let mut account = Account::new(1, "ABCD".to_string());
                                                        // we can also use String::from("ABCD")
    account.balance=400;
    let account_ref = &account;

    print_account(&account_ref);
    print!("{:#?}", account);
    
}
