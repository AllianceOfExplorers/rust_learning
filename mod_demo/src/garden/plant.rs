pub struct Plant {
    pub kind: String,
}

impl Plant {
    pub fn new_plant(k:String) -> Plant {
        Plant { kind: k }
    }

    pub fn kind(&self) -> &str {
       return &self.kind
    }
}

pub fn new_plant(k:String) -> Plant {
    return Plant{kind:k};
}
