use std::time;
use std::thread;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Trainee {
    name: String,
    left: usize,
    right: usize,
}

impl Trainee {
    fn new(name: &str, left: usize, right: usize) -> Trainee {
        Trainee {
            name: name.to_string(),
            //to_string will create a copy of the string where &str points to and give a this new string
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        // underscore tells Rust compiler that we are not going to use this value, so not warning is needed
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);
        
        thread::sleep(time::Duration::new(1, 0));

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let trainees = vec![
        Trainee::new("Bohdan Porokhnavets", 0, 1),
        Trainee::new("Artem Dmytriv", 1, 2),
        Trainee::new("Liubomyr Kotias", 2, 3),
        Trainee::new("Kateryna Dubska", 3, 4),
        Trainee::new("Petro Bratash", 0, 4),
    ];

    let handles: Vec<_> = trainees.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();


    for h in handles {
        h.join().unwrap();
    }
}

//
//    fn new(name: String) -> Trainee {
//
// if we had name: String that we would have to do like this
// which looks worth
//
// in our case extra copy is not such a big of a deal
//
//    let p1 = Trainee::new("Sigmund Freyd".to_string());
//    let p2 = Trainee::new("Mark Manson".to_string());
//    let p3 = Trainee::new("Les Poderevynskii".to_string());
//    let p4 = Trainee::new("Danylo Yanevskii".to_string());
//    let p5 = Trainee::new("Andriy Hubchak".to_string());
//
//    or without new() at all
//    
//    let p1 = Trainee ( name: "Sigmund Freyd".to_string() };
//    let p2 = Trainee ( name: "Mark Manson".to_string() };
//    let p3 = Trainee ( name: "Les Poderevynskii".to_string() };
//    let p4 = Trainee ( name: "Danylo Yanevskii".to_string() };
//    let p5 = Trainee ( name: "Andriy Hubchak".to_string() };
