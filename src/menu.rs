// libraries
use std::io;
use clearscreen;

// Modules
use crate::util::view_inventory;
use crate::util::check_inventory;


// Functions
pub fn display_menu() {
    loop {
        // Clear the console
        clearscreen::clear().expect("Console failed to clear.");
        // Display the menu options
        println!("Inventory Management System");
        println!("1. View Items in inventory");
        println!("2. Add Item to inventory");
        println!("3. Update Item in Inventory");
        println!("4. Remove Item from inventory");
        println!("5. Exit");

        // Prompt the user for their choice
        println!("Please enter your choice:");
        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        // Handle the user's choice
        if choice.trim() == "1" {
            view_inventory::view_inventory();
        } else if choice.trim() == "2" {
            check_inventory::check_inventory();
        } else if choice.trim() == "3" {
            check_inventory::check_inventory();
        } else if choice.trim() == "4" {
            check_inventory::check_inventory();
        } else if choice.trim() == "5" {
            println!("Exiting...");
            break;
        } else {
            println!("Invalid choice, please try again.");
        }
        
        // test the user's choice
        // println!("You chose: {choice}");
    }


}