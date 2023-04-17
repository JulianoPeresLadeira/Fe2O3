use std::cell::{RefCell, RefMut};
use std::sync::{Arc, Mutex};
use std::{thread, time};
use std::thread::JoinHandle;

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
    access: Arc<Mutex<Factory>>,
    tickets: Vec<u16>,
    name: String
}

impl Worker {
    fn new(access: Arc<Mutex<Factory>>, name: String) -> Self {
        let tickets = Vec::with_capacity(100);
        Worker {access, tickets, name}
    }

    fn fetch_ticket(&mut self) {
        let mut factory = self.access.lock().unwrap();
        let mut ticketer = factory.get_ticketer();
        let ticket = ticketer.get_ticket();
        self.tickets.push(ticket);
    }

    fn print_tickets(&self) {
        println!("I am {} and I got the following tickets:", self.name);
        for ticket in self.tickets.iter() {
            println!("{}", ticket);    
        }
    }
}

fn spawn_worker(name: String, factory_access: Arc<Mutex<Factory>>) -> JoinHandle<Worker> {
    println!("Creating thread");
    thread::spawn(
        move || {
            println!("Starting work of {}", name);
            let mut worker = Worker::new(factory_access, name.clone());
            for _ in 0..50 {
                worker.fetch_ticket();
            }
            println!("Ending work of {}", name);
            worker
        }
    )
}

fn get_worker(join_handle: JoinHandle<Worker>) -> Option<Worker> {
    println!("Joining...");
    match join_handle.join() {
        Ok(worker) => Some(worker),
        Err(_) => None 
    }
}

fn main() {
    let factory = Factory::new();
    let factory_pointer = Arc::new(Mutex::new(factory));
    let names = vec!["Emrys", "Craig", "Mathias", "Prithvi", "Andrés"];
    
    let mut handles = vec![];

    for name in names {
        let join_handle = spawn_worker(String::from(name), factory_pointer.clone());
        handles.push(join_handle);
    }

    for handle in handles {
        let worker_option = get_worker(handle);
        worker_option.unwrap().print_tickets();
    }
}
