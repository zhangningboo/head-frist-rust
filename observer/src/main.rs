use std::cell::RefCell;
use std::rc::Rc;

// 定义观察者 trait
trait Observer {
    fn update(&self, message: &str);
}

// 定义主题 trait
trait Subject {
    fn register_observer<T: Observer + 'static>(&mut self, observer: Rc<RefCell<T>>);
    fn remove_observer<T: Observer + 'static>(&mut self, observer: &Rc<RefCell<T>>);
    fn notify_observers(&self, message: &str);
}

// 具体主题
struct ConcreteSubject {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
}

impl ConcreteSubject {
    fn new() -> Self {
        ConcreteSubject {
            observers: Vec::new(),
        }
    }
}

impl Subject for ConcreteSubject {
    // 注册观察者时，自动转换为 trait object
    fn register_observer<T: Observer + 'static>(&mut self, observer: Rc<RefCell<T>>) {
        self.observers.push(observer.clone());
    }

    // 移除观察者时，将 observer 转换为 trait object 形式进行比较
    fn remove_observer<T: Observer + 'static>(&mut self, observer: &Rc<RefCell<T>>) {
        // 使用 clone 自动类型转换为 trait object
        let observer_trait: Rc<RefCell<dyn Observer>> = observer.clone();
        self.observers.retain(|o| !Rc::ptr_eq(o, &observer_trait));
    }

    fn notify_observers(&self, message: &str) {
        for observer in &self.observers {
            observer.borrow().update(message);
        }
    }
}

// 具体观察者 A
struct ConcreteObserverA;

impl Observer for ConcreteObserverA {
    fn update(&self, message: &str) {
        println!("Observer A received message: {}", message);
    }
}

// 具体观察者 B
struct ConcreteObserverB;

impl Observer for ConcreteObserverB {
    fn update(&self, message: &str) {
        println!("Observer B received message: {}", message);
    }
}

fn main() {
    // 创建主题
    let mut subject = ConcreteSubject::new();

    // 创建观察者
    let observer_a = Rc::new(RefCell::new(ConcreteObserverA));
    let observer_b = Rc::new(RefCell::new(ConcreteObserverB));

    // 注册观察者
    subject.register_observer(observer_a.clone());
    subject.register_observer(observer_b.clone());

    // 通知观察者
    subject.notify_observers("Hello Observers!");

    // 移除一个观察者
    subject.remove_observer(&observer_a);
    
    // 再次通知
    subject.notify_observers("Another message after removing A");
}