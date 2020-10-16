use std::thread;

struct Trainee {
    name: String,
}

impl Trainee {
    fn new(name: &str) -> Trainee {
        Trainee {
            name: name.to_string(),
            //to_string will create a copy of the string where &str points to and give a this new string
        }
    }

    fn eat(&self) {
        println!("{} is eating.", self.name);
        
        thread::sleep_ms(1000);

        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let trainees = vec![
        Trainee::new("Bohdan Porokhnavets"),
        Trainee::new("Artem Dmytriv"),
        Trainee::new("Liubomyr Kotias"),
        Trainee::new("Kateryna Dubska"),
        Trainee::new("Petro Bratash"),
    ];

    for p in &trainees {
        p.eat();
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
