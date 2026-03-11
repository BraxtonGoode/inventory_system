# Overview

The software that is created in this Rust project is a simple inventory management system. It allows users to add items to the inventory, view the current inventory, and save the inventory data to a CSV file. The software is designed to be a basic demonstration of Rust's capabilities for file I/O, data manipulation, and user interaction.

The purpose of writing this software was to learn and demonstrate my understanding of the Rust programming language. I wanted to create a practical application that would allow me to explore Rust's syntax, features, and libraries while also creating something useful. This project helped me gain hands-on experience with Rust's file handling, data structures, and some of the error handling capabilities.

{Provide a link to your YouTube demonstration. It should be a 4-5 minute demo of the software running and a walkthrough of the code. Focus should be on sharing what you learned about the language syntax.}

[Software Demo Video](http://youtube.link.goes.here)

# Development Environment

I used Visual Studio Code as my primary code editor, along with the Rust extension for syntax highlighting and code completion. I also used Git for version control and GitHub for repository management.

The software is written in Rust, a systems programming language known for its safety and performance. I utilized the standard Rust library for file I/O operations and CSV handling.
I also used the `csv` crate for easier CSV file manipulation and the `serde` crate for serialization and deserialization of data structures. I also used `clearscreen` crate to clear the terminal screen for better user experience.

# Useful Websites

- [Rust Documentation](https://doc.rust-lang.org/stable/)
- [Learn Rust](https://rust-lang.org/learn/)
- [ChatGPT](https://chatgpt.com/)
- No link but using cargo doc --open was useful aswell.

# Future Work

- Item 1: The biggest improvement is to make it a more generic inventory since all you can add are items. It would be nice to be able to add other types of inventory such as services or digital products. which would require some changes to the data structure and how the inventory is managed.

- Item 2: Figuring out how to create global variables in Rust would be helpful for managing the inventory data across different functions and modules. This would allow for easier access and manipulation of the inventory data without having to pass it around as arguments.

- Item 3: Implementing more error handling would be huge for the program i did some but there could be more to ensure that the program can handle unexpected inputs or situations gracefully without crashing.