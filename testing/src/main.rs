
#[derive(Debug)]
struct Account {
    balance: i32
}


fn first_muts(accounts: &mut Vec<Account>) -> Option<&mut Account>{
    if accounts.len()>1{
        Some(&mut accounts[0])
    }
    else{
        None
    }
}

fn main() {
    let mut accounts: Vec<Account> = vec![
        Account { balance: 0 },
        Account { balance: 10 }
    ];

    match first_muts(&mut accounts) {

        Some(value) => {
            value.balance = 30;
            println!("{:#?}",value);
        }

        None => {
            println!("nothing")
        }
        
    }
    
    // Add code here:
}