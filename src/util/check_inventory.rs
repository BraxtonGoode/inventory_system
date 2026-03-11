// check inventory for add, update, and remove functions
// Libraries
use std::io;
use clearscreen;

// modules
use crate::util::view_inventory;
use crate::actions::add_inventory;
use crate::actions::update_inventory;
use crate::actions::delete_inventory;

// functions
// (*** check_inventory, item_exists_in_inventory ***)



// Description: The check_inventory function is responsible for checking if an item already exists in the inventory before allowing 
// the user to add, update, or delete it. It prompts the user for the item name and checks against the existing inventory. 
// If the item exists, it offers options to update or delete it. If it does not exist, it proceeds to add the new item.
// Args: None
// Returns: None
// Notes: This function enhances the user experience by preventing duplicate entries and providing a clear workflow for managing inventory items.
pub fn check_inventory() {
    // Clear the console and prompt user for item name
    clearscreen::clear().expect("Console failed to clear.");
    println!("Checking inventory...");
    println!("--------------------");
    println!("Check if item is in inventory first?");     
    let mut item_name = String::new();
    io::stdin()
        .read_line(&mut item_name)
        .expect("Failed to read line");
    let item_name = item_name.trim().to_string();

    // Check if item exists in inventory
    if item_exists_in_inventory(&item_name) {
        println!("⚠️  Item '{}' exists in inventory!", item_name);
        println!("Would you like to update, or delete Item? (type update or delete)");
        loop {
            // Prompt user for update or delete choice
            let mut choice = String::new();
            io::stdin()
                .read_line(&mut choice)
                .expect("Failed to read line");

            let choice = choice.trim().to_lowercase();
            
            if choice == "delete" {
                println!("you chose delete");
                delete_inventory::delete_inventory(&item_name);
                break;
            } else if choice == "update" {
                println!("you chose update");
                update_inventory::update_inventory(&item_name);
                break;
            } else {
                println!("Invalid choice. Please type 'update' or 'delete'.");
            }
        }
        
    } else {
        println!("✅ '{}' is a new item. Proceeding to add...", item_name);
        add_inventory::add_inventory(&item_name);
    }


}



// Description: The item_exists_in_inventory function checks if a given item name already exists in the inventory by reading the inventory data from the CSV file.
// Args: item_name - a string slice representing the name of the item to check for existence in the inventory.
// Returns: A boolean value indicating whether the item exists in the inventory (true) or not (false).
// Notes: This function is used to prevent duplicate entries in the inventory and to guide the user towards updating or deleting existing items instead of adding duplicates.
pub fn item_exists_in_inventory(item_name: &str) -> bool {
    match view_inventory::read_inventory_from_csv() {
        Ok(existing_items) => {
            existing_items.iter().any(|item| {
                item.name.to_lowercase() == item_name.to_lowercase()
            })
        }
        Err(e) => {
            println!("❌ Error reading inventory: {}", e);
            println!("💡 Make sure ./data/inventory.csv exists and is readable.");
            false
        }
    }
}