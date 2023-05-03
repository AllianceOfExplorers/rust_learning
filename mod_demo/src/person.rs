
pub mod person {
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
}