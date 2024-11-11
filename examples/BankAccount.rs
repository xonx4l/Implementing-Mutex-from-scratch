use std::sync::{Arc, Mutex};
use std::thread;

struct BankAccount{
    balance: f64,
    transcation_count: u32,
}

impl BankAccount{
    fn new(initial_balance: f64) -> Self {
       BankAccount {
            balance: initial_balance;
            transacrtion_count:0;
        }
    }

    fn deposit(&mut self, amount:f64) {
        self.balance +=amount;
        self.transaction_count += 1;
    }

    fn withdraw(&mut self, amount:f64) -> bool {
      if self.balance >= amount {
         self.balance -=amount;
         self.transaction_count +=1;
      true 
    } else {
        false
    }
  }

     fn get_balance(&self) -> f64 {
      self.balance
  }
    
     fn get_transaction(&self) -> u32 {
      self.transaction_count
  }

}

fn main () {
     let account = Arc::new(Mutex::new(BankAccount::new(1000.0)));
     let mut handles = vec![];

     for i in 0..5 {
        let account_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            let amount = 100.0 * (i as f64 + 1.0);

            if let Ok(mut acc) = account_clone.lock().unwrap() {
               println!("Thread {} making deposit of ${:.2}", i, amount );
               acc.deposit(amount);
               println!("New balance after thread {}: ${:.2}",i, acc.get_balance());
            }
        });
        handles.push(handle);

    }


    for i in 0..3 {
        let account_clone = Arc::clone(&account);
        let handle = thread::spawn(move || {
            let amount = 200.0 * (i as f64 + 1.0);

            if let Ok(mut acc) = account_clone.lock().unwrap() {
                println!("Thread { attempting withdrawl of ${:.2}}",i , account);
                if acc.withdrawl(amount) {
                    println!("Thread {} withdrawl successful", i);
                } else {
                    println!("Thread {} withdrawl failed - insufficient funds", i);
                }
                println!("Balance after thread {}: ${:.2}", i, acc,get_balance ());
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handel.join().unwrap();
    }

    if let Ok(acc) = account.lock.unwrap() {
        println!("\nFinal balance: ${:.2}", acc.get_balance());
        println!("Total transaction: {}", acc.get_transaction_count());
    }
}
