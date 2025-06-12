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

    fn deposit(&mut self, amount: i32){
        self.balance+=amount;
    }

    fn withdraw(&mut self, amount: i32){
        self.balance-=amount;
    }
    fn summary(&self) -> String{
        //println!("Account ID:{}, Holder name:{}, balance:{}", self.id,self.holder,self.balance);
        format!("Account ID:{}, Holder name:{}, balance:{}", self.id,self.holder,self.balance)
        //format! forms a string!
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

    fn add_account(&mut self,account:Account) {
        self.accounts.push(account);
    }

    fn total_balance(&self) -> i32{
        self.accounts.iter().map(|account| account.balance).sum()
        //self.accounts.iter().map(|item| item.balance).sum()
    }

    fn summary(&self) -> Vec<String>{
        self.accounts.iter().map(|account| account.summary()).collect() //or collect::<Vec<String>>()
    }
}

fn new_balance(account:&mut Account, amount:i32 ){
    account.balance=amount;
}



fn main() {
    let mut bank = Bank::new();
    let mut account = Account::new(1, "ABCD".to_string());
                                                        // we can also use String::from("ABCD")
    new_balance(&mut account, 1000);
    account.id=7000;
    account.holder="cat".to_string();

    let mut account1 = Account::new(300, "ABCD".to_string());
    account1.deposit(1000);

    bank.add_account(account);
    bank.accounts[0].withdraw(100);
    bank.add_account(account1);

    println!("{:#?}", bank);
    println!("{}",bank.accounts[0].summary());
    println!("\ntotal balance in bank:{}",bank.total_balance());
    println!("\ntotal balance in bank:{:#?}",bank.summary());
}
