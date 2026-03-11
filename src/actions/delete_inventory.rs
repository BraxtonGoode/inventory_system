// delete inventory item
// Libraries
use std::io;
use clearscreen;

// Modules
use crate::util::view_inventory;


// Functions
// (*** delete_inventory, find_item_by_name, delete_item ***)

// Description: The delete_inventory function is responsible for deleting an item from the inventory. 
// It takes the item name as an argument, finds the item in the inventory, and prompts the user for confirmation before deleting it.
// Args: item_name - a string slice representing the name of the item to be deleted from the inventory.
// Returns: None
// Notes: This function manages the delete capabilities.
pub fn delete_inventory(item_name: &str) {
    // clear console
    clearscreen::clear().expect("Console failed to clear.");
    println!("Deleting inventory...");
    println!("--------------------");
    // find item by name then delete item
    find_item_by_name(item_name);
}

// Description: The find_item_by_name function searches for an item in the inventory by its name. 
// If the item is found, it calls the delete_item function to handle the deletion process. If the item is not found, it informs the user.
// Args: name - a string slice representing the name of the item to be searched for in the inventory.
// Returns: None
// Notes: This function reads the inventory from the CSV file and iterates through the items to find a match based on the item name, ignoring case sensitivity.
fn find_item_by_name(name: &str) {
    match view_inventory::read_inventory_from_csv() {
        Ok(items) => {
            for item in items {
                if item.name.to_lowercase() == name.to_lowercase() {
                    delete_item(item.id, &item.name, item.quantity, item.price);
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

// Description: The delete_item function prompts the user for confirmation before deleting an item from the inventory.
// If the user confirms, it reads the existing inventory, removes the specified item, and writes the updated inventory back to the CSV file.
// Args: item_id - a u32 representing the ID of the item to be deleted, item_name - a string slice representing the name of the item, 
// quantity - a u32 representing the quantity of the item, price - a f64 representing the price of the item.
// Returns: None
// Notes: This function ensures that the user has the opportunity to cancel the deletion if they choose not to proceed, 
// and it handles the actual removal of the item from the inventory and updates the CSV file accordingly.
fn delete_item(item_id: u32, item_name: &str, quantity: u32, price: f64){
    // prints current item details and prompts user for confirmation before deletion
    println!("Current details:");
    println!("Id: {}, Name: {}, Quantity: {}, Price: ${}", item_id, item_name, quantity, price);
    println!("---------------------");
    loop {
        // confirm deletion with user
        println!("Do you want to delete this item? (y/n)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input = input.trim().to_lowercase();
        if input == "y" {
            break;
        } else if input == "n" {
            println!("Delete cancelled.");
            return;
        } else {
            println!("Invalid input. Please enter 'y' or 'n'.");
        }
    }
    // delete the item from inventory
    match view_inventory::read_inventory_from_csv() {
        Ok(mut items) => {
            items.retain(|item| item.id != item_id);
            // write updated items back to CSV
            let file_path = "./data/inventory.csv";
            let mut writer = csv::Writer::from_path(file_path).expect("Failed to open inventory.csv for writing");
            for item in items {
                writer.serialize(item).expect("Failed to write item to CSV");
            }
            writer.flush().expect("Failed to flush CSV writer");
            println!("✅ Item deleted from inventory successfully!");
        }
        Err(err) => {
            println!("Error reading inventory: {}", err);
           
        }
    }
}