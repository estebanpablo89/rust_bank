
#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}


impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account { 
            id: id,
            holder: holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![]}
    }
}

fn print_account(account: &Account) {
    println!("Account: {:#?}", account);
}

fn change_account(account: &mut Account) {
    account.balance += 10;
}

fn main() {
    let mut account = Account::new(1, String::from("Alice"));

    change_account(&mut account);

    print!("{:#?}", account)
}
