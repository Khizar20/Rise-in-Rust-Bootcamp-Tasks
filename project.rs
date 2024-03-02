use std::io;
use std::io::Write;

struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

impl Product {
    fn new(name: String, description: String, price: f64, quantity: u32) -> Self {
        Product {
            name,
            description,
            price,
            quantity,
        }
    }
}

struct Inventory {
    products: Vec<Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: Vec::new(),
        }
    }

    fn add_product(&mut self, product: Product) {
        self.products.push(product);
    }

    fn delete_product(&mut self, index: usize) -> Option<Product> {
        if index < self.products.len() {
            Some(self.products.remove(index))
        } else {
            None
        }
    }

    fn edit_product(&mut self, index: usize, name: String, description: String, price: f64, quantity: u32) -> Result<(), &'static str> {
        if index >= self.products.len() {
            println!("Invalid index");
            return Err("Invalid index");
        }
    
        let product = &mut self.products[index];
        product.name = name;
        product.description = description;
        product.price = price;
        product.quantity = quantity;
        
        Ok(())
    }
    
    

    fn generate_report(&self) {
        println!();
        println!("══════════════════════");
        println!("GENERATING REPORT.....");
        println!("══════════════════════");
        println!();
        if self.products.is_empty() {
            println!("No items in the inventory.");
        } else {
            for (index, product) in self.products.iter().enumerate() {
                println!("Product {}: {}", index + 1, product.name);
                println!("Description: {}", product.description);
                println!("Price: ${:.2}", product.price);
                println!("Quantity: {}\n", product.quantity);
            }
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();

    loop {
        println!("╔═══════════════════════════════════╗");
        println!("║               Menu                ║");
        println!("╠═══════════════════════════════════╣");
        println!("║ 1. Add Product                    ║");
        println!("║                                   ║"); 
        println!("║ 2. Delete Product                 ║");
        println!("║                                   ║"); 
        println!("║ 3. Edit Product                   ║");
        println!("║                                   ║"); 
        println!("║ 4. Generate Report                ║");
        println!("║                                   ║"); 
        println!("║ 5. Exit                           ║");
        println!("╚═══════════════════════════════════╝");
        println!();
        print!("Enter your choice: ");
        io::stdout().flush().unwrap(); // Flush stdout to ensure prompt is displayed immediately

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input");
                continue;
            }
        };

        match input {
            1 => add_product(&mut inventory),
            2 => delete_product(&mut inventory),
            3 => edit_product(&mut inventory),
            4 => inventory.generate_report(),
            5 => {
                println!();
                println!("═════════════");
                println!("EXITING......");
                println!("═════════════");
                println!();
                break;},
            _ => println!("Invalid input"),
        }
    }
}

fn add_product(inventory: &mut Inventory) {

    println!();
    println!("══════════════════════════════");
    println!("WELCOME TO ADD PRODUCT SECTION");
    println!("══════════════════════════════");
    println!();
    println!("Enter product name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter product description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read input");

    println!("Enter product price:");
    let mut price_str = String::new();
    io::stdin().read_line(&mut price_str).expect("Failed to read input");
    let price: f64 = price_str.trim().parse().expect("Invalid price");

    println!("Enter product quantity:");
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str).expect("Failed to read input");
    let quantity: u32 = quantity_str.trim().parse().expect("Invalid quantity");

    let product = Product::new(name.trim().to_string(), description.trim().to_string(), price, quantity);
    inventory.add_product(product);
    println!("Product added successfully!");
}

fn delete_product(inventory: &mut Inventory) {
    println!();
    println!("═════════════════════════════════");
    println!("WELCOME TO DELETE PRODUCT SECTION");
    println!("═════════════════════════════════");
    println!();
    println!("Enter the index of the product to delete:");
    let mut index_str = String::new();
    io::stdin().read_line(&mut index_str).expect("Failed to read input");
    let index: usize = index_str.trim().parse().expect("Invalid index");

    if let Some(_) = inventory.delete_product(index - 1) {
        println!("Product deleted successfully!");
    } else {
        println!("Invalid index");
    }
}

fn edit_product(inventory: &mut Inventory) {
    println!();
    println!("════════════════════════════════");
    println!("WELCOME TO EDIT PRODUCT SECTION");
    println!("════════════════════════════════");
    println!();
    println!("Enter the index of the product to edit:");
    let mut index_str = String::new();
    io::stdin().read_line(&mut index_str).expect("Failed to read input");
    let index: usize = index_str.trim().parse().expect("Invalid index");

    if index == 0 || index > inventory.products.len() {
        println!("Invalid index");
        return;
    }

    println!("Enter new product name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");

    println!("Enter new product description:");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read input");

    println!("Enter new product price:");
    let mut price_str = String::new();
    io::stdin().read_line(&mut price_str).expect("Failed to read input");
    let price: f64 = price_str.trim().parse().expect("Invalid price");

    println!("Enter new product quantity:");
    let mut quantity_str = String::new();
    io::stdin().read_line(&mut quantity_str).expect("Failed to read input");
    let quantity: u32 = quantity_str.trim().parse().expect("Invalid quantity");

    if let Err(err) = inventory.edit_product(index - 1, name.trim().to_string(), description.trim().to_string(), price, quantity) {
        println!("{}", err);
    } else {
        println!("Product edited successfully!");
    }
}

