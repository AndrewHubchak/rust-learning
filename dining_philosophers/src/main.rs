struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            //to_string will create a copy of the string where &str points to and give a this new string
        }
    }

    fn eat(&self) {
        println!("{} is done eating.", self.name);
    }
}

fn main() {
    let p1 = Philosopher::new("Sigmund Freyd");
    let p2 = Philosopher::new("Mark Manson");
    let p3 = Philosopher::new("Les Poderevynskii");
    let p4 = Philosopher::new("Danylo Yanevskii");
    let p5 = Philosopher::new("Andriy Hubchak");
}

//
//    fn new(name: String) -> Philosopher {
//
// if we had name: String that we would have to do like this
// which looks worth
//
// in our case extra copy is not such a big of a deal
//
//    let p1 = Philosopher::new("Sigmund Freyd".to_string());
//    let p2 = Philosopher::new("Mark Manson".to_string());
//    let p3 = Philosopher::new("Les Poderevynskii".to_string());
//    let p4 = Philosopher::new("Danylo Yanevskii".to_string());
//    let p5 = Philosopher::new("Andriy Hubchak".to_string());
//
//    or without new() at all
//    
//    let p1 = Philosopher ( name: "Sigmund Freyd".to_string() };
//    let p2 = Philosopher ( name: "Mark Manson".to_string() };
//    let p3 = Philosopher ( name: "Les Poderevynskii".to_string() };
//    let p4 = Philosopher ( name: "Danylo Yanevskii".to_string() };
//    let p5 = Philosopher ( name: "Andriy Hubchak".to_string() };
