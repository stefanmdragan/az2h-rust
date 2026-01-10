use std::io;

#[derive(Debug, Clone)]
struct Bill {
    description: String,
    amount: f64,
}

struct BillManager {
    bills: Vec<Bill>,
}

impl BillManager {
    fn new() -> Self {
        BillManager { bills: Vec::new() }
    }

    fn add_bill(&mut self, bill: Bill) {
        self.bills.push(bill);
    }

    fn remove_bill(&mut self, index: usize) -> bool {
        if index < self.bills.len() {
            self.bills.remove(index);
            true
        } else {
            false
        }
    }

    fn get_bills(&self) -> Vec<&Bill> {
        // get borrowed references to bills
        self.bills.iter().collect()
    }
}

fn get_line() -> String {
    let mut line = String::new();
    while io::stdin().read_line(&mut line).is_err() {
        eprintln!("Error reading input. Please try again.");
    }
    let line = line.trim().to_owned();
    line
}

mod menu {
    use crate::{Bill, BillManager, get_line};

    pub fn add_bill(bills: &mut BillManager) {
        println!("Enter bill description:");
        let description = get_line();
        if description.is_empty() {
            eprintln!("Description cannot be empty. Bill not added.");
            return;
        }

        println!("Enter bill amount:");
        let amount_input = get_line();
        if amount_input.is_empty() {
            eprintln!("Amount cannot be empty. Bill not added.");
            return;
        }

        let amount: f64 = match amount_input.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid amount. Bill not added.");
                return;
            }
        };

        let bill = Bill {
            description,
            amount,
        };
        bills.add_bill(bill);
        println!("Bill added successfully.");
    }

    pub fn remove_bill(bills: &mut BillManager) {
        println!("Enter the index of the bill to remove:");
        let index_input = get_line();
        let index: usize = match index_input.parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Invalid index. No bill removed.");
                return;
            }
        };

        if bills.remove_bill(index - 1) {
            println!("Bill removed successfully.");
        } else {
            eprintln!("No bill found at the given index. No bill removed.");
        }
    }

    pub fn view_bills(bills: &BillManager) {
        let all_bills = bills.get_bills();
        if all_bills.is_empty() {
            println!("No bills to display.");
            return;
        }

        println!("Bills:");
        for (i, bill) in all_bills.iter().enumerate() {
            println!("{}. {} - ${:.2}", i + 1, bill.description, bill.amount);
        }
    }
}

enum MainMenu {
    AddBill,
    ViewBills,
    RemoveBill,
    Exit,
}

impl MainMenu {
    fn from_input(input: &str) -> Option<Self> {
        match input {
            "1" => Some(Self::AddBill),
            "2" => Some(Self::ViewBills),
            "3" => Some(Self::RemoveBill),
            "4" => Some(Self::Exit),
            _ => None,
        }
    }

    fn display() {
        println!("");
        println!("Menu:");
        println!("1. Add Bill");
        println!("2. View Bills");
        println!("3. Remove Bill");
        println!("4. Exit");
        println!("");
        print!("Please select an option: ");
        io::Write::flush(&mut io::stdout()).expect("Failed to flush stdout");
    }
}

fn main() {
    let mut bill_manager = BillManager::new();

    // main menu loop
    loop {
        MainMenu::display();

        let input = get_line();
        match MainMenu::from_input(input.as_str()) {
            Some(MainMenu::AddBill) => {
                menu::add_bill(&mut bill_manager);
            }
            Some(MainMenu::ViewBills) => {
                menu::view_bills(&bill_manager);
            }
            Some(MainMenu::RemoveBill) => {
                menu::view_bills(&bill_manager);
                menu::remove_bill(&mut bill_manager);
            }
            Some(MainMenu::Exit) | None => {
                println!("Exiting...");
                break;
            }
        }
    }
}
