// libraries
use std::io;
use clearscreen;
use std::time::Duration;
use std::thread;



// Modules
use crate::util::save_item_to_csv;


// Functions

pub fn add_inventory(item_name: &str) {
    clearscreen::clear().expect("Console failed to clear.");
    println!("Adding item to inventory...");
    println!("--------------------");
    // item name
    println!("Item name: {}", item_name);
    // item quantity
    let quantity = get_item_quantity();
    // item price
    let price = get_item_price();
    // display item details
    display_item_details(item_name, quantity, price);
    // save item to CSV
    save_item_to_csv::save_item_to_csv_new_id(item_name, quantity, price);



}
fn get_item_quantity() -> u32 {
    loop {
        println!("Enter quantity (whole number):");
        let mut quantity_input = String::new();
        io::stdin()
            .read_line(&mut quantity_input)
            .expect("Failed to read line");

        match quantity_input.trim().parse::<u32>() {
            Ok(num) if num > 0 => return num,
            Ok(_) => println!("Quantity must be greater than 0!"),
            Err(_) => println!("Please enter a valid whole number!"),
        }
    }
}

fn get_item_price() -> f64 {
    loop {
        println!("Enter price (e.g., 19.99):");
        let mut price_input = String::new();
        io::stdin()
            .read_line(&mut price_input)
            .expect("Failed to read line");

        match price_input.trim().parse::<f64>() {
            Ok(num) if num > 0.0 => return num,
            Ok(_) => println!("Price must be greater than 0!"),
            Err(_) => println!("Please enter a valid number!"),
        }
    }
}

fn total_value(quantity: u32, price: f64) -> f64 {
    quantity as f64 * price
}

fn display_item_details(name: &str, quantity: u32, price: f64) {
    clearscreen::clear().expect("Console failed to clear.");
    println!("Item Details:");
    println!("--------------------");
    println!("Item Name: {}", name);
    println!("Quantity: {}", quantity);
    println!("Price: ${:.2}", price);
    println!("Total Value: ${:.2}", total_value(quantity, price));
    println!("--------------------");
    println!("Press Enter to return to the main menu.");
    let mut enter = String::new();
    io::stdin().read_line(&mut enter).expect("Failed to read input.");
    thread::sleep(Duration::from_secs(1));    
}

