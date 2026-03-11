// saving an item to a csv file
// Libraries
use csv::Writer;

// Modules
use crate::models::item::Item;
use crate::util::view_inventory;

// Functions
// (*** save_item_to_csv_new_id, update_item_to_csv, get_next_id ***)


// Description: The get_next_id function reads the existing inventory from the CSV file and 
// determines the next available ID for a new item by finding the maximum existing ID and adding 1 to it. If there are no items in the inventory, it starts with ID 1.
// Args: None
// Returns: u32 - The next available ID for a new inventory item.
// Notes: This function ensures that each new item added to the inventory has a unique ID.
fn get_next_id() -> u32 {
    match view_inventory::read_inventory_from_csv() {
        Ok(items) => {
            // Find the maximum ID in the existing items and add 1 to it. If there are no items, start with ID 1.
            items.iter().map(|item| item.id).max().unwrap_or(0) + 1
        }
        Err(_) => 1,
    }
}


// Description: The save_item_to_csv_new_id function creates a new Item struct with a unique ID and the provided name, quantity, and price. 
// It then reads the existing items from the CSV file, adds the new item to the list, and writes all items back to the CSV file.
// Args: name - a string slice representing the name of the item, quantity - a u32 representing the quantity of the item, price - a f64 representing the price of the item.
// Returns: None
// Notes: This function handles both adding a new item to the inventory and ensuring that the CSV file is updated with the new item while preserving existing items.
pub fn save_item_to_csv_new_id(name: &str, quantity: u32, price: f64) {
    // Generate the next ID for the new item
    let next_id = get_next_id();
    let new_item = Item::new(next_id, name.to_string(), quantity, price);
    // read existing items from CSV
    let mut existing_items = match view_inventory::read_inventory_from_csv() {
        Ok(items) => items,
        Err(_) => Vec::new(),
    };

    // add the new item
    existing_items.push(new_item);

    // write all items back to CSV
    let file_path = "./data/inventory.csv";
    let mut writer = Writer::from_path(file_path).expect("Failed to open inventory.csv for writing");

    for item in existing_items {
        writer.serialize(item).expect("Failed to write item to CSV");
    }
    writer.flush().expect("Failed to flush CSV writer");
    println!("✅ Item added to inventory successfully!");

}





// Description: The update_item_to_csv function takes an existing item's ID and updated details (name, quantity, price), 
// reads the current inventory from the CSV file, updates the item with the matching ID, and writes all items back to the CSV file.
// Args: id - a u32 representing the ID of the item to update, name - a string slice representing the updated name of the item, 
// quantity - a u32 representing the updated quantity of the item, price - a f64 representing the updated price of the item.
// Returns: None
// Notes: This function allows for updating existing items in the inventory by matching the item ID. 
// It ensures that the CSV file reflects the updated item details while preserving all other items.
pub fn update_item_to_csv(id: u32,name: &str, quantity: u32, price: f64) {
    // read exiting items from CSV
    let mut existing_items = match view_inventory::read_inventory_from_csv() {
        Ok(items) => items,
        Err(_) => Vec::new(),
    };
    // find and update the item with the given id
    for item in existing_items.iter_mut() {
        if item.id == id {
            item.name = name.to_string();
            item.quantity = quantity;
            item.price = price;
            break;
        }
    }
    // write all items back to CSV
    let file_path = "./data/inventory.csv";
    let mut writer = Writer::from_path(file_path).expect("Failed to open inventory.csv for writing");

    for item in existing_items {
        writer.serialize(item).expect("Failed to write item to CSV");
    }
    writer.flush().expect("Failed to flush CSV writer");
    // success message
    println!("✅ Item updated in inventory successfully!");
}