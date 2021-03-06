use bank_account::BankAccount;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bank {
    data_path: String,
    bank_accounts: HashMap<String, BankAccount>,
}

impl Bank {
    pub fn new(data_path: String) -> Self {
        let mut bank_accounts: HashMap<String, BankAccount> = HashMap::new();

        _ = match File::open(&data_path) {
            Err(_) => {
                println!("No existing data, initializing");
                match File::create(&data_path) {
                    Ok(file) => file,
                    Err(e) => panic!("Could not create new file, Err: {}", e),
                }
            }
            Ok(mut file) => {
                let mut data = String::new();
                file.read_to_string(&mut data).unwrap();
                match serde_json::from_str(&data[..]) {
                    Ok(s) => bank_accounts = s,
                    Err(e) => println!(
                        "File {} is empty, leaving bank_accounts empty, Err: {}",
                        &data_path, e
                    ),
                }
                file
            }
        };
        Self {
            data_path,
            bank_accounts,
        }
    }

    // Creates bank account and overwrites stored file, this does not scale and is expensive
    // For persistence and scaling I would use a proper DB with sharding
    // I could also append entries at the end of the file, but short on time for now
    //
    // I'm also cloning "name" when a borrow and pointer to BankAccount.name would be preferable
    // But causing problems with the serializer and lifetime
    pub fn create_account(&mut self, name: &String, balance: usize) {
        if self.bank_accounts.contains_key(name) {
            panic!("Account already exists");
        }
        self.bank_accounts
            .insert(name.clone(), BankAccount::new(name.clone(), balance));

        self.save_data();
    }

    pub fn transfer(&mut self, from: &String, to: &String, amount: usize) -> Result<(), String> {
        let from_funds: &mut usize = match self.bank_accounts.get_mut(from) {
            Some(i) => i.balance_mut(),
            None => return Err(format!("Could not retrieve balance from {from}")),
        };

        if *from_funds < amount {
            return Err(format!("Not enough funds in {from} to transfer"));
        }

        *from_funds -= amount;

        let to_funds: &mut usize = match self.bank_accounts.get_mut(to) {
            Some(i) => i.balance_mut(),
            None => return Err(format!("Could not retrieve balance from {from}")),
        };

        *to_funds += amount;

        self.save_data();

        Ok(())
    }

    pub fn view_balance(&self, account_name: &String) {
        println!(
            "{}'s current balance is: {}",
            account_name,
            self.bank_accounts.get(account_name).unwrap().balance()
        );
    }

    fn save_data(&self) {
        let mut file = File::create(&self.data_path).unwrap();
        match file.write_all(
            serde_json::to_string(&self.bank_accounts)
                .unwrap()
                .as_bytes(),
        ) {
            Ok(_) => (),
            Err(e) => println!("Could not write serialized data to file, Err: {}", e),
        }
    }
}

mod bank_account {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct BankAccount {
        name: String,
        balance: usize,
    }

    impl BankAccount {
        pub fn new(name: String, balance: usize) -> Self {
            Self { name, balance }
        }

        pub fn balance(&self) -> &usize {
            &self.balance
        }

        pub fn balance_mut(&mut self) -> &mut usize {
            &mut self.balance
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{fs, path::Path};
    const TEST_FILE_CREATION_PATH: &str = "data/test_file_creation";
    const TEST_BANK_NEW_PATH: &str = "data/test_bank_new";
    const NEW_INSTANCE_OLD_DATA_PATH: &str = "data/new_instance_old_data";
    const TRANSFER_PATH: &str = "data/transfer";
    const TRANSFER_FAIL_PATH: &str = "data/transfer_fail";

    #[test]
    fn test_bank_new() {
        _ = Bank::new(TEST_BANK_NEW_PATH.to_string());
        assert!(Path::new(TEST_BANK_NEW_PATH).exists());
        fs::remove_file(TEST_BANK_NEW_PATH).unwrap();
    }

    #[test]
    fn test_file_creation() {
        let mut bank = Bank::new(TEST_FILE_CREATION_PATH.to_string());
        bank.create_account(&String::from("Phil"), 5);
        bank.create_account(&String::from("Rizza"), 1000);
        fs::remove_file(TEST_FILE_CREATION_PATH).unwrap();
    }

    #[test]
    fn new_instance_old_data() {
        _ = initializing(NEW_INSTANCE_OLD_DATA_PATH);
        {
            let mut bank = Bank::new(NEW_INSTANCE_OLD_DATA_PATH.to_string());
            bank.create_account(&String::from("Phil"), 5);
            bank.create_account(&String::from("Rizza"), 1000);
        }
        let mut bank = Bank::new(NEW_INSTANCE_OLD_DATA_PATH.to_string());
        bank.create_account(&String::from("Carmen"), 999);
        bank.create_account(&String::from("Camila"), 77777);

        assert!(bank.bank_accounts.len() == 4);
        for (name, bank_account) in bank.bank_accounts {
            println!("Account: {}, Balance: {}", name, bank_account.balance());
        }
    }

    #[test]
    fn transfer() {
        let mut bank = initializing(TRANSFER_PATH);
        bank.create_account(&String::from("Phil"), 5);
        bank.create_account(&String::from("Rizza"), 1000);
        bank.transfer(&"Phil".to_string(), &"Rizza".to_string(), 5)
            .unwrap();

        assert!(*bank.bank_accounts.get("Phil").unwrap().balance() == 0);
        assert!(*bank.bank_accounts.get("Rizza").unwrap().balance() == 1005);
    }

    #[test]
    #[should_panic]
    fn transfer_fail() {
        let mut bank = initializing(TRANSFER_FAIL_PATH);
        bank.create_account(&String::from("Phil"), 5);
        bank.create_account(&String::from("Rizza"), 1000);
        bank.transfer(&"Phil".to_string(), &"Rizza".to_string(), 6)
            .unwrap();
    }

    fn initializing(path: &str) -> Bank {
        match fs::remove_file(path) {
            Ok(_) => println!("File removed"),
            Err(_) => println!("No file to remove"),
        }
        Bank::new(path.to_string())
    }
}
