use std::cell::RefCell;
use std::rc::Rc;

use dyn_clone::{self, DynClone};

// 使用宏定义可以被克隆的 trait object
dyn_clone::clone_trait_object!(Command);

// 定义 Command trait，并自动派生 Clone 支持
trait Command: DynClone {
    fn execute(&self);
    fn undo(&self);
}

#[derive(Clone)]
struct Light {
    is_on: bool,
}

impl Light {
    fn new() -> Self {
        Light { is_on: false }
    }

    fn on(&mut self) {
        self.is_on = true;
        println!("Light is ON");
    }

    fn off(&mut self) {
        self.is_on = false;
        println!("Light is OFF");
    }
}

#[derive(Clone)]
struct LightOnCommand {
    light: Rc<RefCell<Light>>,
}

impl LightOnCommand {
    fn new(light: Rc<RefCell<Light>>) -> Self {
        LightOnCommand { light }
    }
}

impl Command for LightOnCommand {
    fn execute(&self) {
        self.light.borrow_mut().on();
    }

    fn undo(&self) {
        self.light.borrow_mut().off();
    }
}

#[derive(Clone)]
struct LightOffCommand {
    light: Rc<RefCell<Light>>,
}

impl LightOffCommand {
    fn new(light: Rc<RefCell<Light>>) -> Self {
        LightOffCommand { light }
    }
}

impl Command for LightOffCommand {
    fn execute(&self) {
        self.light.borrow_mut().off();
    }

    fn undo(&self) {
        self.light.borrow_mut().on();
    }
}

struct RemoteControl {
    command: Option<Box<dyn Command>>,
    history: Vec<Box<dyn Command>>,
}

impl RemoteControl {
    fn new() -> Self {
        RemoteControl {
            command: None,
            history: vec![],
        }
    }

    fn set_command(&mut self, command: Box<dyn Command>) {
        self.command = Some(command);
    }

    fn press_button(&mut self) {
        if let Some(ref cmd) = self.command {
            cmd.execute();
            self.history.push(dyn_clone::clone_box(cmd.as_ref()));
        }
    }

    fn press_undo(&self) {
        if let Some(last_cmd) = self.history.last() {
            last_cmd.undo();
        }
    }
}

fn main() {
    let light = Rc::new(RefCell::new(Light::new()));

    let light_on = Box::new(LightOnCommand::new(light.clone()));
    let light_off = Box::new(LightOffCommand::new(light.clone()));

    let mut remote = RemoteControl::new();

    remote.set_command(light_on);
    remote.press_button();

    remote.set_command(light_off);
    remote.press_button();

    remote.press_undo();
}