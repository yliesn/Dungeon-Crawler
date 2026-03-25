use rand::Rng;
use crate::map::Map;

// --- Trait partagé ---

pub trait Entity {
    fn position(&self) -> (usize, usize);
    fn hp(&self) -> i32;
    fn attack(&self) -> i32;
    fn is_alive(&self) -> bool {
        self.hp() > 0
    }
    fn symbol(&self) -> char;
}

// --- Joueur ---

pub struct Player {
    pub x: usize,
    pub y: usize,
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
}

impl Player {
    pub fn new(x: usize, y: usize) -> Self {
        Player {
            x,
            y,
            hp: 30,
            max_hp: 30,
            attack: 8,
        }
    }

    /// Tente de déplacer le joueur. Retourne true si le mouvement est valide.
    pub fn try_move(&mut self, dx: i32, dy: i32, map: &Map) -> bool {
        let nx = self.x as i32 + dx;
        let ny = self.y as i32 + dy;

        if nx < 0 || ny < 0 {
            return false;
        }

        let nx = nx as usize;
        let ny = ny as usize;

        if ny >= map.height || nx >= map.width {
            return false;
        }

        if map.is_walkable(nx, ny) {
            self.x = nx;
            self.y = ny;
            true
        } else {
            false
        }
    }

    pub fn heal(&mut self, amount: i32) {
        self.hp = (self.hp + amount).min(self.max_hp);
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }
}

impl Entity for Player {
    fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    fn hp(&self) -> i32 {
        self.hp
    }
    fn attack(&self) -> i32 {
        self.attack
    }
    fn symbol(&self) -> char {
        '@'
    }
}

// --- Types d'ennemis ---

#[derive(Clone, Debug)]
pub enum EnemyKind {
    Goblin,
    Troll,
    Skeleton,
}

impl EnemyKind {
    pub fn stats(&self) -> (i32, i32) {
        // (hp, attack)
        match self {
            EnemyKind::Goblin   => (10, 4),
            EnemyKind::Troll    => (20, 7),
            EnemyKind::Skeleton => (14, 5),
        }
    }

    pub fn symbol(&self) -> char {
        match self {
            EnemyKind::Goblin   => 'g',
            EnemyKind::Troll    => 'T',
            EnemyKind::Skeleton => 's',
        }
    }

    pub fn name(&self) -> &str {
        match self {
            EnemyKind::Goblin   => "Goblin",
            EnemyKind::Troll    => "Troll",
            EnemyKind::Skeleton => "Squelette",
        }
    }
}

// --- Ennemi ---

pub struct Enemy {
    pub x: usize,
    pub y: usize,
    pub hp: i32,
    pub attack: i32,
    pub kind: EnemyKind,
}

impl Enemy {
    pub fn new(x: usize, y: usize, kind: EnemyKind) -> Self {
        let (hp, attack) = kind.stats();
        Enemy { x, y, hp, attack, kind }
    }

    pub fn take_damage(&mut self, amount: i32) {
        self.hp -= amount;
    }

    /// Déplacement aléatoire dans une des 4 directions
    pub fn wander(&mut self, map: &Map, occupied: &[(usize, usize)]) {
        let mut rng = rand::thread_rng();
        let directions = [(0i32, -1i32), (0, 1), (-1, 0), (1, 0)];
        let mut candidates: Vec<(usize, usize)> = Vec::new();

        for (dx, dy) in &directions {
            let nx = self.x as i32 + dx;
            let ny = self.y as i32 + dy;

            if nx < 0 || ny < 0 {
                continue;
            }
            let nx = nx as usize;
            let ny = ny as usize;

            if ny < map.height && nx < map.width
                && map.is_walkable(nx, ny)
                && !occupied.contains(&(nx, ny))
            {
                candidates.push((nx, ny));
            }
        }

        if !candidates.is_empty() {
            let idx = rng.gen_range(0..candidates.len());
            self.x = candidates[idx].0;
            self.y = candidates[idx].1;
        }
    }

    pub fn name(&self) -> &str {
        self.kind.name()
    }
}

impl Entity for Enemy {
    fn position(&self) -> (usize, usize) {
        (self.x, self.y)
    }
    fn hp(&self) -> i32 {
        self.hp
    }
    fn attack(&self) -> i32 {
        self.attack
    }
    fn symbol(&self) -> char {
        self.kind.symbol()
    }
}
