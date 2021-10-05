
struct Hex {
    pub elevation: u8,
    pub occupied: bool,
}

impl Hex {
    pub fn new() -> Hex {
        Hex { elevation: 0, occupied: false, }
    }
}

struct Board {
    
}

fn main() {
    println!("Hello, world!");
    let hex = Hex::new();
}
