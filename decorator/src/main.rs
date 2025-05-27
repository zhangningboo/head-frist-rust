// trait 接口
trait Beverage {
    fn cost(&self) -> f32;
    fn description(&self) -> String;
}

struct Espresso;

impl Beverage for Espresso {
    fn cost(&self) -> f32 {
        1.99
    }

    fn description(&self) -> String {
        String::from("Espresso")
    }
}

struct DarkRoast;

impl Beverage for DarkRoast {
    fn cost(&self) -> f32 {
        0.99
    }

    fn description(&self) -> String {
        String::from("Dark Roast Coffee")
    }
}

// 使用 Box<dyn Beverage> 实现装饰者
struct MilkFoam {
    beverage: Box<dyn Beverage>,
}

impl MilkFoam {
    fn new(beverage: Box<dyn Beverage>) -> Box<Self> {
        Box::new(Self { beverage })
    }
}

impl Beverage for MilkFoam {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.10
    }

    fn description(&self) -> String {
        format!("{} + Milk Foam", self.beverage.description())
    }
}

struct Mocha {
    beverage: Box<dyn Beverage>,
}

impl Mocha {
    fn new(beverage: Box<dyn Beverage>) -> Box<Self> {
        Box::new(Self { beverage })
    }
}

impl Beverage for Mocha {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.20
    }

    fn description(&self) -> String {
        format!("{} + Mocha", self.beverage.description())
    }
}

struct Soy {
    beverage: Box<dyn Beverage>,
}

impl Soy {
    fn new(beverage: Box<dyn Beverage>) -> Box<Self> {
        Box::new(Self { beverage })
    }
}

impl Beverage for Soy {
    fn cost(&self) -> f32 {
        self.beverage.cost() + 0.15
    }

    fn description(&self) -> String {
        format!("{} + Soy", self.beverage.description())
    }
}

fn main() {
    // 基础咖啡
    let mut beverage: Box<dyn Beverage> = Box::new(Espresso);
    println!("{} ${:.2}", beverage.description(), beverage.cost());

    // 添加摩卡
    beverage = Box::new(Mocha { beverage });
    println!("{} ${:.2}", beverage.description(), beverage.cost());

    // 添加豆奶
    beverage = Box::new(Soy { beverage });
    println!("{} ${:.2}", beverage.description(), beverage.cost());

    // 另一个例子
    let mut beverage2: Box<dyn Beverage> = Box::new(DarkRoast);
    beverage2 = Box::new(Mocha { beverage: beverage2 });
    beverage2 = Box::new(Mocha { beverage: beverage2 });
    beverage2 = Box::new(MilkFoam { beverage: beverage2 });
    println!("{} ${:.2}", beverage2.description(), beverage2.cost());
}