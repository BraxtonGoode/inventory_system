// This file defines the Item struct, which represents an inventory item with fields for id, name, quantity, and price.

// Libraries
use serde::{Serialize, Deserialize};

// Modules
#[derive(Debug, Clone, Serialize, Deserialize)]


// Struct definition for an inventory item
pub struct Item {
    pub id: u32,
    pub name: String,
    pub quantity: u32,
    pub price: f64,
}

// Implement methods for the Item struct
impl Item {
    // Constructor function
    // This is used to create a new Item instance with the provided id, name, quantity, and price.
    pub fn new(id: u32, name: String, quantity: u32, price: f64) -> Item {
        Item {
            id,
            name,
            quantity,
            price,
        }
    }

    // Calculate total value (computed property)
    pub fn total_value(&self) -> f64 {
        self.quantity as f64 * self.price
    }
}