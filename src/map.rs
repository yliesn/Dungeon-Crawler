use rand::Rng;

#[derive(Clone, PartialEq, Debug)]
pub enum Cell {
    Wall,
    Floor,
    Exit,
}

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Vec<Cell>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = vec![vec![Cell::Floor; width]; height];

        // Bordures = murs
        for x in 0..width {
            cells[0][x] = Cell::Wall;
            cells[height - 1][x] = Cell::Wall;
        }
        for y in 0..height {
            cells[y][0] = Cell::Wall;
            cells[y][width - 1] = Cell::Wall;
        }

        // Murs intérieurs aléatoires (~20% des cases)
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if rng.gen_ratio(1, 5) {
                    cells[y][x] = Cell::Wall;
                }
            }
        }

        // Sortie en bas à droite (zone dégagée)
        cells[height - 2][width - 2] = Cell::Exit;
        cells[height - 3][width - 2] = Cell::Floor;
        cells[height - 2][width - 3] = Cell::Floor;

        // S'assurer que le spawn du joueur est libre
        cells[1][1] = Cell::Floor;
        cells[1][2] = Cell::Floor;
        cells[2][1] = Cell::Floor;

        Map { width, height, cells }
    }

    pub fn is_walkable(&self, x: usize, y: usize) -> bool {
        matches!(self.cells[y][x], Cell::Floor | Cell::Exit)
    }

    pub fn is_exit(&self, x: usize, y: usize) -> bool {
        self.cells[y][x] == Cell::Exit
    }
}
