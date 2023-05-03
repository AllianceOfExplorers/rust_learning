pub mod plant;

pub struct Garden {
    area: isize,
}

impl Garden {
    pub fn get_area(&self) -> isize {
        self.area
    }
}

pub fn new_garden() -> Garden {
    Garden { area: 10 }
}