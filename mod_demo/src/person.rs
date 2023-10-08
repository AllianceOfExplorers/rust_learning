
use crate::garden::plant;

pub mod person {
    use crate::garden::plant;
    use crate::garden::plant::new_plant;

    pub struct Person {
        pub name: String,
        pub age: isize,
        pub feal: bool,
    }

    impl Person {
        pub fn new_person(name: String,age: isize,feal:bool) -> Person {
            Person{name:name,age:age,feal:feal}
        }

        pub fn name(&self) -> &str {
            return &(self.name[..]);
        }
    }

    pub fn deal() {
        println!("hello person");
    }

    fn work() {
        let s = plant::new_plant("s".to_string());
    }
}