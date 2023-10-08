
pub mod garden;
pub mod person;
pub mod solution;

use std::io;
use std::path::Path;
use crate::garden::plant;
fn main() {
   // let p = crate::person::New("henry".to_string(),18,true);
   // println!("name is {:?}",p);
   //panic!("crash test");

    let p = plant::new_plant("tomato".to_string());
    println!("kind is {}",p.kind());
    person::person::Person::new_person("heng".to_string(), 18, true);
}

fn move_all(src:&Path, dst:&Path) ->io::Result<()> {
    for entry in src.read_dir()? {
        let e = entry?;

    }

    Ok(())
}