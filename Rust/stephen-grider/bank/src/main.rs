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

    fn deposit(&mut self, amount:&i32) {
        self.balance +=amount;
    }

    fn withdrawals(&mut self , amount:&i32){
        self.balance -= amount;
    }

    fn summary(&self) -> String {
        format!("{} has a balance {}", self.holder, self.balance)
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

    fn add_account (&mut self, account :Account) {
        self.accounts.push(account);
    }
    
    fn total_balance(&self) -> i32 {
        self.accounts.iter().map(|account | account.balance).sum()
    }

    fn summary(&self) -> Vec<String> {
        self.accounts.iter().map(|account | account.summary()).collect::<Vec<String>>()
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

fn make_and_print_account()  {// &Account { lifetime paramater
    let account = Account::new(1, String::from("Aditya"));
    println!("{:#?}",account);
    //&account
}

fn main() {
    //println!("Hello, world!");

    let mut bank = Bank::new();
    let mut account = Account::new(1, String::from("Aditya"));
    println!("{:#?}", account.summary());

    let mut dd_account = Account::new(2, String::from("Ambani"));
    dd_account.deposit(&43);
    dd_account.withdrawals(&32);
    bank.add_account(dd_account);
    //let other_bank = bank;
    //
    println!("Demo Account {:#?}", bank.summary());
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
    //let Acc = other_bank.accounts;
    //println!("{:#?}", other_bank.accounts);

    let mut d_account = Account::new(1, String::from("me"));
    let account_ref = &mut d_account;
    d_account.balance = 10;
    //Change_account(account_ref);
    println!("{:#?}", account);

    println!("{:#?}", other_bank.summary());



}
