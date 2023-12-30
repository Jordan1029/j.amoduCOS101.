//a struct for laptop brands
struct Laptop {
    brand: String,
    price: f64,
}

// Implement a method for the Laptop struct to calculate total cost
impl Laptop {
    // Calculate total cost for a given quantity
    fn calculate_total_cost(&self, quantity: u32) -> f64 {
        self.price * f64::from(quantity)
    }
}

fn main() {
    let hp_laptop = Laptop {
        brand: String::from("HP"),
        price: 650000.0,
    };

    let ibm_laptop = Laptop {
        brand: String::from("IBM"),
        price: 755000.0,
    };

    let toshiba_laptop = Laptop {
        brand: String::from("Toshiba"),
        price: 550000.0,
    };

    let dell_laptop = Laptop {
        brand: String::from("Dell"),
        price: 850000.0,
    };

    // Calculate the total cost for each brand when a customer purchases 3 laptops
    let total_cost_hp = hp_laptop.calculate_total_cost(3);
    let total_cost_ibm = ibm_laptop.calculate_total_cost(3);
    let total_cost_toshiba = toshiba_laptop.calculate_total_cost(3);
    let total_cost_dell = dell_laptop.calculate_total_cost(3);

    // Calculate the overall total cost
    let overall_total_cost = total_cost_hp + total_cost_ibm + total_cost_toshiba + total_cost_dell;
    println!("Total cost for HP laptops: {}", total_cost_hp);
    println!("Total cost for IBM laptops: {}", total_cost_ibm);
    println!("Total cost for Toshiba laptops: {}", total_cost_toshiba);
    println!("Total cost for Dell laptops: {}", total_cost_dell);
    println!("Overall total cost: {}", overall_total_cost);
}