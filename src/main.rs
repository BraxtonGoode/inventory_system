// The main function for the inventory system

// modules
mod menu;
mod actions;
mod models;
mod util;


// Functions
// (***main***)

// Description: The main function serves as the entry point for the inventory management system. It calls the display_menu function to start the user interface.
// Args: None
// Returns: None
// Notes: The main function is simple and primarily responsible for initializing the application by displaying the menu
fn main(){
    menu::display_menu();
}