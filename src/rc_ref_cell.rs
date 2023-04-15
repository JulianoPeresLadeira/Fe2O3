use std::cell::{RefCell, RefMut};
use std::rc::Rc;

struct Ticketer {
    next_ticket: u16
}

impl Ticketer {
    fn new() -> Self {
        Ticketer {
            next_ticket: 0
        }
    }

    fn get_ticket(&mut self) -> u16 {
        let ticket = self.next_ticket;
        self.next_ticket += 1;
        ticket
    }
}

struct Factory {
    ticketer: RefCell<Ticketer>
}

impl Factory {
    fn new() -> Self {
        Factory {
            ticketer: RefCell::new(Ticketer::new())
        }
    }

    fn get_ticketer(&mut self) -> RefMut<Ticketer> {
        self.ticketer.borrow_mut()
    }
}

struct Worker {
    access: Rc<RefCell<Factory>>,
    name: String
}

impl Worker {
    fn new(access: Rc<RefCell<Factory>>, name: String) -> Self {
        Worker {access, name}
    }

    fn print_ticket(&mut self) {
        let mut binding = self.access.borrow_mut();
        let mut ticketer = binding.get_ticketer();
        let ticket = ticketer.get_ticket();

        println!("I am {}, I got the ticket: {}", self.name, ticket);
    }
}

fn main() {
    let factory = Factory::new();
    let factory_pointer = Rc::new(RefCell::new(factory));
    let mut craig = Worker::new(factory_pointer.clone(), String::from("Craig"));
    let mut emrys = Worker::new(factory_pointer.clone(), String::from("Emrys"));
    craig.print_ticket();
    emrys.print_ticket();
    craig.print_ticket();
    emrys.print_ticket();
    craig.print_ticket();
    emrys.print_ticket();
    craig.print_ticket();
    emrys.print_ticket();
}
