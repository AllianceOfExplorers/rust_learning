
pub mod garden;
pub mod person;

use crate::garden::plant;
fn main() {
   // let p = crate::person::New("henry".to_string(),18,true);
   // println!("name is {:?}",p);
   panic!("crash test");

    let p = plant::new_plant("tomato".to_string());
    println!("kind is {}",p.kind());
    person::person::Person::new_person("heng".to_string(), 18, true);
}
