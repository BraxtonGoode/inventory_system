// update inventory item
// Libraries
use std::io;
use clearscreen;


// Modules
use crate::util::view_inventory;
use crate::util::save_item_to_csv;


// Functions
// (*** update_inventory, find_item_by_name, update_item_details ***)

// Description: The update_inventory function is responsible for updating an existing item in the inventory.
// It takes the item name as an argument, finds the item in the inventory, and prompts the user to update the item details.
// Args: item_name - a string slice representing the name of the item to be updated in the inventory.
// Returns: None
// Notes: This function manages the update capabilities.
pub fn update_inventory(item_name: &str) {
    clearscreen::clear().expect("Console failed to clear.");
    println!("Updating inventory...");
    println!("--------------------");
    // find item by name
    find_item_by_name(item_name);
}

// Description: The find_item_by_name function searches for an item in the inventory by its name.
// If the item is found, it calls the update_item_details function to handle the update process. If the item is not found, it informs the user.
// Args: name - a string slice representing the name of the item to be searched for in the inventory.
// Returns: None
// Notes: This function reads the inventory from the CSV file and iterates through the items to find a match based on the item name, ignoring case sensitivity.
fn find_item_by_name(name: &str) {
    match view_inventory::read_inventory_from_csv() {
        Ok(items) => {
            for item in items {
                if item.name.to_lowercase() == name.to_lowercase() {
                    update_item_details(item.id, &item.name, item.quantity, item.price);
                    return;
                }
            }
            println!("Item '{}' not found in inventory.", name);
        }
        Err(err) => {
            println!("Error reading inventory: {}", err);
        }
    }
}


// Description: The update_item_details function prompts the user for confirmation before updating an item in the inventory.
// If the user confirms, it allows the user to update the name, quantity, and price of the item, and then saves the updated item details to the CSV file.
// Args: item_id - a u32 representing the ID of the item to be updated, item_name - a string slice representing the name of the item,
// new_quantity - a u32 representing the current quantity of the item, new_price - a f64 representing the current price of the item.
// Returns: None
// Notes: This function ensures that the user has the opportunity to cancel the update if they choose not to proceed, 
// and it handles the actual updating of the item details in the inventory and updates the CSV file accordingly.
fn update_item_details(item_id: u32, item_name: &str, new_quantity: u32, new_price: f64){
    // prints current item details and prompts user for confirmation before updating
    println!("Current details:");
    println!("Id: {}, Name: {}, Quantity: {}, Price: ${}", item_id, item_name, new_quantity, new_price);
    println!("---------------------");
    loop {
        println!("Do you want to update this item? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();
        if input == "y" {
            break;
        } else if input == "n" {
            println!("Update cancelled.");
            return;
        } else {
            println!("Invalid input. Please enter 'y' or 'n'.");
        }
    }
    loop {
        // Asks user which detail they would like to update and prompts for new value, then saves updated item details to CSV file
        println!("Which detail would you like to update?");
        println!("1. Name");
        println!("2. Quantity");
        println!("3. Price");
        println!("4. Done");
        println!("Enter the number corresponding to your choice:");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim();
        match choice {
            "1" => {
                // New name input
                println!("Enter new name:");
                let mut new_name = String::new();
                io::stdin().read_line(&mut new_name).expect("Failed to read input");
                let new_name = new_name.trim();
                println!("Updating name...");
                save_item_to_csv::update_item_to_csv(item_id, new_name, new_quantity, new_price);
                println!("Name updated to '{}'", new_name);
            }
            "2" => {
                // new quantity input
                println!("Enter new quantity:");
                let mut new_quantity = String::new();
                io::stdin().read_line(&mut new_quantity).expect("Failed to read input");
                let new_quantity: u32 = match new_quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid quantity. Update cancelled.");
                        return;
                    }
                };
                println!("Updating quantity...");
                save_item_to_csv::update_item_to_csv(item_id, item_name, new_quantity, new_price);
                println!("Quantity updated to {}", new_quantity);
            }
            "3" => {
                // New price input
                println!("Enter new price:");
                let mut new_price = String::new();
                io::stdin().read_line(&mut new_price).expect("Failed to read input");
                let new_price: f64 = match new_price.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid price. Update cancelled.");
                        return;
                    }
                };
                println!("Updating price...");
                save_item_to_csv::update_item_to_csv(item_id, item_name, new_quantity, new_price);
                println!("Price updated to ${}", new_price);
            }
            "4" => {
                println!("Update completed.");
                return;
            }
            _ => {
                println!("Invalid choice.. please enter a number between 1 and 4.");
            }
        }
    }
}

