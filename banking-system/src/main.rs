#[derive(Debug,Clone)]
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
    let mut bank = Bank::new();
    let mut account = Account::new(1, "ABCD".to_string());
                                                        // we can also use String::from("ABCD")
    account.balance=400;
    bank.accounts.push(account.clone());

    let account_ref = &mut account; //changing the data through borrowing
    account_ref.balance=7000;


    print_account(&account);
    println!("{:#?}", bank);
    println!("{:#?}", account);
    
}
