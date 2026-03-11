// Purpose: View all items in inventory
// Libraries
use csv::Reader;
use std::error::Error;
use std::fs::File;
use clearscreen;
use std::time::Duration;
use std::thread;

// Modules
use crate::models::item::Item;


// Functions 
// (***read_inventory_from_csv and view_inventory***)

// Description: Reads inventory data from a CSV file and returns a vector of Item structs.
// Args: None
// Returns: Result<Vec<Item>, Box<dyn Error>> - A vector of Item structs on success, or an error on failure.
// Notes: The CSV file should have headers that match the fields of the Item struct (id, name, quantity, price).
pub fn read_inventory_from_csv() -> Result<Vec<Item>, Box<dyn Error>> {
    let file_path = "./data/inventory.csv";
    let file = File::open(file_path)?;  
    let mut reader = Reader::from_reader(file);
    let mut items = Vec::new();  
    
    for result in reader.deserialize() {
        let item: Item = result?;  
        items.push(item);  
    }
    
    Ok(items) 
}

// Description: Clears the console and displays the inventory items read from the CSV file.
// Args: None
// Returns: None
// Notes: This function will clear the console before displaying the inventory. It will also handle any
// errors that occur while reading the inventory and provide user-friendly messages.
pub fn view_inventory() {
    // Clear the console before displaying inventory
    clearscreen::clear().expect("Console failed to clear.");
    println!("Viewing inventory...");
    println!("--------------------");
    // read inventory from CSV and display items
    match read_inventory_from_csv() {
        Ok(items) => {
            // Now you have items to iterate over!
            for (index, item) in items.iter().enumerate() {
                println!("{}: {} (Quantity: {}, Price: ${:.2}, Total Value: ${:.2})", 
                    index + 1, 
                    item.name, 
                    item.quantity, 
                    item.price, 
                    item.total_value());
                println!("--------------------");
            }
            println!("✅ Inventory displayed successfully.");
            // Ask user if they want to return to main menu
            println!("Are you ready to return to the main menu? (y/n)");
            loop {
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).expect("Failed to read input.");
                if input.trim().eq_ignore_ascii_case("y") {
                    println!("Returning to main menu...");
                    thread::sleep(Duration::from_secs(2));
                    break;
                } else if input.trim().eq_ignore_ascii_case("n") {
                    println!("Staying on inventory view. Press Enter to return to main menu.");
                    let mut _dummy = String::new();
                    std::io::stdin().read_line(&mut _dummy).expect("Failed to read input.");
                    break;
                } else {
                    println!("Invalid input. Please enter 'y' or 'n'.");
                }
            }

        }
        Err(e) => {
            println!("❌ Error reading inventory: {}", e);
            println!("💡 Make sure ./data/inventory.csv exists and is readable.");
        }
    }
}
