use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub quantity: u32,
    pub price: f64,
}

impl Item {
    // Constructor function
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