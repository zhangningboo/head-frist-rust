use std::sync::Mutex;
use once_cell::sync::Lazy;

// 定义库存结构体
struct CoffeeInventory {
    beans_count: u32,
}

impl CoffeeInventory {
    fn new() -> Self {
        CoffeeInventory {
            beans_count: 1000, // 初始库存
        }
    }

    fn use_beans(&mut self, amount: u32) {
        if self.beans_count >= amount {
            self.beans_count -= amount;
            println!("Used {} beans. Remaining: {}", amount, self.beans_count);
        } else {
            println!("Not enough beans! Only {} left.", self.beans_count);
        }
    }

    fn get_stock(&self) -> u32 {
        self.beans_count
    }
}

// 单例实例（Lazy + Mutex）
static INSTANCE: Lazy<Mutex<CoffeeInventory>> = Lazy::new(|| {
    Mutex::new(CoffeeInventory::new())
});

// 提供访问接口
pub fn get_inventory() -> &'static Mutex<CoffeeInventory> {
    &INSTANCE
}

fn main() {
    // 获取库存单例
    let inv1 = get_inventory();
    let inv2 = get_inventory();

    // 测试调用
    {
        let mut inv = inv1.lock().unwrap();
        inv.use_beans(200);
        println!("Stock: {}", inv.get_stock());
    }

    {
        let mut inv = inv2.lock().unwrap();
        inv.use_beans(300);
        println!("Stock: {}", inv.get_stock());
    }
}