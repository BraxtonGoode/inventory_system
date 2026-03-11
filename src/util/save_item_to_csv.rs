// saving an item to a csv file
// Libraries
use csv::Writer;

// Modules
use crate::models::item::Item;
use crate::util::view_inventory;

// Functions
fn get_next_id() -> u32 {
    match view_inventory::read_inventory_from_csv() {
        Ok(items) => {
            items.iter().map(|item| item.id).max().unwrap_or(0) + 1
        }
        Err(_) => 1,
    }
}

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