
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

fn main() {
    let account = Account::new(1, String::from("Alice"));

    let account_ref = &account;

    print_account(account_ref);

    print!("{:#?}", account_ref)
}
