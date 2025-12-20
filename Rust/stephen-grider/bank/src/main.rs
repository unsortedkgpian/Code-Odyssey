#[derive(Debug)]
struct Account {
    id:u32, 
    balance:i32,
    holder:String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self{
        Account {
            id,
            balance:0,
            holder,
        }
    }
}


#[derive(Debug)]
struct Bank{
    accounts:Vec<Account>,
}

impl Bank {
    fn new() -> Self{
        Bank{accounts:vec![]}
    }
}

fn print_account(account:Account) -> Account {
    println!("{:#?}", account);
    account
}

fn Print_account(account: &Account) {
    println!("{:#?}", account);
}

fn print_holder(holder:String){
    println!("{:#?}",holder);
}

fn change_account(mut account:Account) -> Account {
    account.balance = 10;
    account
} 

fn Change_account(account: &mut Account){
    account.balance = 100;
}


fn main() {
    //println!("Hello, world!");

    let bank = Bank::new();
    let mut account = Account::new(1, String::from("Aditya"));

    //let other_bank = bank;

    println!("New bank {:#?}", bank);
    //println!("New Account {:#?}", account);
    //account = print_account(account);
    //account = print_account(account);
    //println!("{:#?}", account);
    
    let account_ref = &account;
    //print_holder(account_ref.holder);

    Print_account(&account);
    println!("{:#?}",account);

    
    

    let mut demo_account = Account::new(
        1,
        String::from("me")
    );
    // print_holder(demo_account.holder);
    //println!("{:#?}", demo_account.holder);
    //print_account(demo_account);

    println!("{:#?}", demo_account);
    demo_account = change_account(demo_account);
    println!("{:#?}", demo_account);
    Change_account(&mut demo_account);

    println!("{:#?}", demo_account);

    //let list_of_account = vec![demo_account];
    //println!("{:#?}", demo_account);
    let other_bank = Bank::new();
    let Acc = other_bank.accounts;
    //println!("{:#?}", other_bank.accounts);

    let mut d_account = Account::new(1, String::from("me"));
    let account_ref = &mut d_account;
    d_account.balance = 10;
    Change_account(account_ref);
    println!("{:#?}", account);





}
