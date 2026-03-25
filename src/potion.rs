pub struct Potion {
    pub x: usize,
    pub y: usize,
    pub heal_amount: i32,
    pub picked_up: bool,
}

impl Potion {
    pub fn new(x: usize, y: usize) -> Self {
        Potion {
            x,
            y,
            heal_amount: 15,
            picked_up: false,
        }
    }
}
