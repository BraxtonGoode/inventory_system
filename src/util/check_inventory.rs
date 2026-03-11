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
pub fn check_inventory() {
    clearscreen::clear().expect("Console failed to clear.");
    println!("Checking inventory...");
    println!("--------------------");
    println!("Check if item is in inventory first?");     
    let mut item_name = String::new();
    io::stdin()
        .read_line(&mut item_name)
        .expect("Failed to read line");
    let item_name = item_name.trim().to_string();


    if item_exists_in_inventory(&item_name) {
        println!("⚠️  Item '{}' exists in inventory!", item_name);
        println!("Would you like to update, or delete Item? (type update or delete)");
        loop {
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