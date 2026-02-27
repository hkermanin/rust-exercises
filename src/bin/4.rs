#[derive(Debug)]
#[allow(dead_code)]
enum Status {
    InStock,
    OnWay(u32),
    Discontinued,
}

#[derive(Debug)]
struct Item {
    name: String,
    quantity: u32,
    price: f64,
    status: Status,
}

impl Item {
    fn new(name: &str, quantity: u32, price: f64, status: Status) -> Self {
        Self {
            name: name.to_string(),
            quantity,
            price,
            status,
        }
    }

    fn total_value(&self) -> f64 {
        self.price * self.quantity as f64
    }
}

fn main() {
    let mut inventory: Vec<Item> = Vec::new();
    inventory.push(Item {
        name: "laptap".to_string(),
        quantity: 50,
        price: 1000.5,
        status: Status::OnWay(5),
    });
    inventory.push(Item::new("mobile", 100, 560.3, Status::InStock));

    add_item(&mut inventory, "mobile", 20, 40.0, Status::Discontinued);

    show_inventory(&inventory);

    println!("{}", contains_item(&inventory, "mobile"));
    println!(
        "total vlaue of inventory: {}$",
        total_inventory_value(&inventory)
    );
}

fn total_inventory_value(inventory: &[Item]) -> f64 {
    let mut value: f64 = 0.0;
    for item in inventory {
        match &item.status {
            Status::InStock => value += item.total_value(),
            Status::OnWay(_) => continue,
            Status::Discontinued => continue,
        }
    }
    value
}

fn contains_item(inventory: &[Item], name: &str) -> bool {
    for i in inventory {
        if i.name == name {
            return true;
        }
    }
    false
}

fn add_item(inventory: &mut Vec<Item>, name: &str, quantity: u32, price: f64, status: Status) {
    inventory.push(Item::new(name, quantity, price, status));
}

fn show_inventory(inventory: &[Item]) {
    for item in inventory {
        println!("{:?}", item);
    }
}
