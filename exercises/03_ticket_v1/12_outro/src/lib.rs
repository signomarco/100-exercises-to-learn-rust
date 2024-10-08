// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

fn validate_product_name(name: &String) -> bool {
    if name.is_empty() || name.len() > 300 {
        false
    } else { true }
}

fn validate_quantity(quantity: &u16) -> bool {
    if quantity <= &0 {
        false
    } else { true }
}

fn validate_unit_price(price: &u16) -> bool {
    if price <= &0 {
        false
    } else { true }
}

pub struct Order {
    product_name: String,
    quantity: u16,
    unit_price: u16
}

impl Order {
    // Constructor
    pub fn new(name: String, quantity: u16, unit_price: u16) -> Self {
        if !validate_product_name(&name) {
            panic!();
        }
        if !validate_quantity(&quantity) {
            panic!()
        }
        if !validate_unit_price(&unit_price) {
            panic!()
        }

        Self {
            product_name: name,
            quantity,
            unit_price
        }
    }

    // setters
    pub fn set_product_name(&mut self, name: String) {
        if !validate_product_name(&name) {
            panic!();
        }
        self.product_name = name;
    }

    pub fn set_quantity(&mut self, quantity: u16) {
        if !validate_quantity(&quantity) {
            panic!()
        }
        self.quantity = quantity;
    }

    pub fn set_unit_price(&mut self, unit_price: u16) {
        if !validate_unit_price(&unit_price) {
            panic!()
        }
        self.unit_price = unit_price;
    }

    // getters
    pub fn product_name(&self) -> &String {
        &self.product_name
    }

    pub fn quantity(&self) -> &u16 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u16 {
        &self.unit_price
    }

    // Utilities
    pub fn total(&self) -> u16 {
        self.quantity * self.unit_price
    }
}