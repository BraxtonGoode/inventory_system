// This file adds items to the inventory.
// libraries
use std::io;
use clearscreen;
use std::time::Duration;
use std::thread;



// Modules
use crate::util::save_item_to_csv;


// Functions
// (***add_inventory, get_item_quantity, get_item_price, total_value, display_item_details***)


// Description: The add_inventory function is responsible for adding a new item to the inventory. 
// It prompts the user for the item name, quantity, and price, then displays the item details, and then saves the new item to the CSV file using the save_item_to_csv module.
// Args: item_name - a string slice representing the name of the item to be added to the inventory.
// Returns: None
// Notes: This function ensures that the user provides valid input for quantity and price, and it calculates the total value of the item based 
// on the quantity and price before saving it to the inventory.
pub fn add_inventory(item_name: &str) {
    // clear console
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

// Description: The get_item_quantity function prompts the user to enter a quantity for the item being added to the inventory.
// It validates the input to ensure that it is a positive whole number and returns the quantity as a u32.
// Args: None
// Returns: u32 - The quantity of the item to be added to the inventory.
// Notes: This function uses a loop to continuously prompt the user until valid input is received, ensuring that the quantity is a positive integer.
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

// Description: The get_item_price function prompts the user to enter a price for the item being added to the inventory.
// It validates the input to ensure that it is a positive number and returns the price as a f64.
// Args: None
// Returns: f64 - The price of the item to be added to the inventory.
// Notes: This function uses a loop to continuously prompt the user until valid input is received, ensuring that the price is a positive number 
// and is in a valid format (e.g., 19.99).
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

// Description: The total_value function calculates the total value of an item based on its quantity and price.
// Args: quantity - a u32 representing the quantity of the item, price - a f64 representing the price of the item.
// Returns: f64 - The total value of the item (quantity multiplied by price).
// Notes: This function is used to calculate the total value of an item in the inventory, which can be useful for displaying item details and for inventory management purposes.
fn total_value(quantity: u32, price: f64) -> f64 {
    quantity as f64 * price
}

// Description: The display_item_details function clears the console and displays the details of an item, including its name, quantity, price, and total value.
// Args: name - a string slice representing the name of the item, quantity - a u32 representing the quantity of the item, price - a f64 representing the price of the item.
// Returns: None
// Notes: This function is used to provide a clear and formatted display of an item's details after the user has entered the information for a new item being added to the inventory. It also prompts the user to press Enter to return to the main menu after viewing the details.
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

