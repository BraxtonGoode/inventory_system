// delete inventory item
// Libraries
use std::io;
use clearscreen;

// Modules
use crate::util::view_inventory;



// Functions
pub fn delete_inventory(item_name: &str) {
    clearscreen::clear().expect("Console failed to clear.");
    println!("Deleting inventory...");
    println!("--------------------");
    // find item by name
    find_item_by_name(item_name);
}


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

fn delete_item(item_id: u32, item_name: &str, quantity: u32, price: f64){
    println!("Current details:");
    println!("Id: {}, Name: {}, Quantity: {}, Price: ${}", item_id, item_name, quantity, price);
    println!("---------------------");
    loop {
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