trait Pizza {
    fn prepare(&self);
    fn bake(&self);
    fn cut(&self);
    fn box_pizza(&self);

    // 默认方法
    fn order_pizza(&self) {
        self.prepare();
        self.bake();
        self.cut();
        self.box_pizza();
    }
}

struct NYStyleCheesePizza;

impl Pizza for NYStyleCheesePizza {
    fn prepare(&self) {
        println!("NY Style Cheese Pizza is being prepared.");
    }

    fn bake(&self) {
        println!("Baking NY Style Cheese Pizza.");
    }

    fn cut(&self) {
        println!("Cutting NY Style Cheese Pizza.");
    }

    fn box_pizza(&self) {
        println!("Boxing NY Style Cheese Pizza.");
    }
}

struct ChicagoStyleCheesePizza;

impl Pizza for ChicagoStyleCheesePizza {
    fn prepare(&self) {
        println!("Chicago Style Cheese Pizza is being prepared.");
    }

    fn bake(&self) {
        println!("Baking Chicago Style Cheese Pizza.");
    }

    fn cut(&self) {
        println!("Cutting Chicago Style Cheese Pizza.");
    }

    fn box_pizza(&self) {
        println!("Boxing Chicago Style Cheese Pizza.");
    }
}

trait PizzaStore {
    // 工厂方法：由子类实现
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza>;

    // 客户端使用的通用方法
    fn order_pizza(&self, pizza_type: String) -> Box<dyn Pizza> {
        let pizza = self.create_pizza(pizza_type.as_str());

        println!("--- Making a {} ---", pizza_type);
        pizza.order_pizza();

        pizza
    }
}

struct NYPizzaStore;

impl PizzaStore for NYPizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        match pizza_type {
            "cheese" => Box::new(NYStyleCheesePizza),
            _ => panic!("Unknown pizza type"),
        }
    }
}

struct ChicagoPizzaStore;

impl PizzaStore for ChicagoPizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        match pizza_type {
            "cheese" => Box::new(ChicagoStyleCheesePizza),
            _ => panic!("Unknown pizza type"),
        }
    }
}

fn main() {
    let ny_store = NYPizzaStore;
    let chicago_store = ChicagoPizzaStore;

    println!("Client ordering NY Style Cheese Pizza:");
    ny_store.order_pizza("cheese".to_string());

    println!("\nClient ordering Chicago Style Cheese Pizza:");
    chicago_store.order_pizza("cheese".to_string());
}