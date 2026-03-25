use rand::Rng;
use crate::map::Map;
use crate::entity::{Enemy, EnemyKind, Entity, Player};
use crate::potion::Potion;

#[derive(PartialEq)]
pub enum GameState {
    Playing,
    Victory,
    GameOver,
}

pub struct Game {
    pub map: Map,
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub potions: Vec<Potion>,
    pub log: Vec<String>,
    pub state: GameState,
    pub turn: u32,
}

impl Game {
    pub fn new() -> Self {
        let map = Map::new(20, 14);
        let player = Player::new(1, 1);

        let enemies = spawn_enemies(&map, 5);
        let potions = spawn_potions(&map, 3);

        Game {
            map,
            player,
            enemies,
            potions,
            log: vec!["Bienvenue dans le donjon ! Atteins la sortie [E].".to_string()],
            state: GameState::Playing,
            turn: 0,
        }
    }

    /// Traite un mouvement du joueur. dx/dy = direction.
    pub fn move_player(&mut self, dx: i32, dy: i32) {
        let nx = self.player.x as i32 + dx;
        let ny = self.player.y as i32 + dy;

        if nx < 0 || ny < 0 { return; }
        let nx = nx as usize;
        let ny = ny as usize;

        // Y a-t-il un ennemi sur la case cible ?
        let enemy_idx = self.enemies.iter().position(|e| e.x == nx && e.y == ny && e.is_alive());

        if let Some(idx) = enemy_idx {
            // Combat
            self.resolve_combat(idx);
        } else {
            // Déplacement normal
            self.player.try_move(dx, dy, &self.map);

            // Ramasser une potion ?
            for potion in self.potions.iter_mut() {
                if !potion.picked_up && potion.x == self.player.x && potion.y == self.player.y {
                    potion.picked_up = true;
                    self.player.heal(potion.heal_amount);
                    self.log.push(format!(
                        "Tu ramasses une potion ! +{} PV (PV : {}/{})",
                        potion.heal_amount, self.player.hp, self.player.max_hp
                    ));
                }
            }

            // Vérifier la sortie
            if self.map.is_exit(self.player.x, self.player.y) {
                self.state = GameState::Victory;
                self.log.push("🏆 Tu as atteint la sortie ! VICTOIRE !".to_string());
                return;
            }
        }

        self.turn += 1;
        self.move_enemies();
        self.check_game_over();
    }

    fn resolve_combat(&mut self, enemy_idx: usize) {
        let mut rng = rand::thread_rng();
        let player_dmg = self.player.attack + rng.gen_range(0..4);
        let enemy_dmg = self.enemies[enemy_idx].attack + rng.gen_range(0..3);

        self.enemies[enemy_idx].take_damage(player_dmg);
        self.player.take_damage(enemy_dmg);

        let enemy_name = self.enemies[enemy_idx].name().to_string();
        let enemy_hp = self.enemies[enemy_idx].hp;

        if !self.enemies[enemy_idx].is_alive() {
            self.log.push(format!(
                "⚔ Tu frappes le {} pour {} dégâts. Il est mort ! (Tu perds {} PV)",
                enemy_name, player_dmg, enemy_dmg
            ));
        } else {
            self.log.push(format!(
                "⚔ Tu frappes le {} pour {} dégâts (PV restants: {}). Il riposte pour {} dégâts !",
                enemy_name, player_dmg, enemy_hp, enemy_dmg
            ));
        }
    }

    fn move_enemies(&mut self) {
        let player_pos = (self.player.x, self.player.y);

        // Liste des positions occupées (joueur + autres ennemis vivants)
        for i in 0..self.enemies.len() {
            if !self.enemies[i].is_alive() {
                continue;
            }

            // Positions occupées par les autres ennemis
            let occupied: Vec<(usize, usize)> = self.enemies.iter()
                .enumerate()
                .filter(|(j, e)| *j != i && e.is_alive())
                .map(|(_, e)| (e.x, e.y))
                .chain(std::iter::once(player_pos))
                .collect();

            let ex = self.enemies[i].x;
            let ey = self.enemies[i].y;

            self.enemies[i].wander(&self.map, &occupied);

            // Si l'ennemi se déplace sur le joueur → attaque
            if self.enemies[i].x == player_pos.0 && self.enemies[i].y == player_pos.1 {
                // Revenir en arrière (pas de superposition)
                self.enemies[i].x = ex;
                self.enemies[i].y = ey;

                let mut rng = rand::thread_rng();
                let dmg = self.enemies[i].attack + rng.gen_range(0..3);
                self.player.take_damage(dmg);
                let name = self.enemies[i].name().to_string();
                self.log.push(format!("💀 Le {} t'attaque pour {} dégâts !", name, dmg));
            }
        }
    }

    fn check_game_over(&mut self) {
        if !self.player.is_alive() {
            self.state = GameState::GameOver;
            self.log.push("☠ Tu es mort... GAME OVER.".to_string());
        }
    }

    /// Garde seulement les N derniers messages du log
    pub fn recent_log(&self, n: usize) -> &[String] {
        let len = self.log.len();
        if len <= n {
            &self.log
        } else {
            &self.log[len - n..]
        }
    }
}

fn spawn_enemies(map: &Map, count: usize) -> Vec<Enemy> {
    let mut rng = rand::thread_rng();
    let kinds = [EnemyKind::Goblin, EnemyKind::Troll, EnemyKind::Skeleton];
    let mut enemies = Vec::new();
    let mut attempts = 0;

    while enemies.len() < count && attempts < 1000 {
        attempts += 1;
        let x = rng.gen_range(1..map.width - 1);
        let y = rng.gen_range(1..map.height - 1);

        // Pas trop près du spawn joueur
        if x < 4 && y < 4 { continue; }
        if !map.is_walkable(x, y) { continue; }
        if enemies.iter().any(|e: &Enemy| e.x == x && e.y == y) { continue; }

        let kind = kinds[rng.gen_range(0..kinds.len())].clone();
        enemies.push(Enemy::new(x, y, kind));
    }

    enemies
}

fn spawn_potions(map: &Map, count: usize) -> Vec<Potion> {
    let mut rng = rand::thread_rng();
    let mut potions = Vec::new();
    let mut attempts = 0;

    while potions.len() < count && attempts < 1000 {
        attempts += 1;
        let x = rng.gen_range(1..map.width - 1);
        let y = rng.gen_range(1..map.height - 1);

        if !map.is_walkable(x, y) { continue; }
        // Pas sur le spawn
        if x == 1 && y == 1 { continue; }
        if potions.iter().any(|p: &Potion| p.x == x && p.y == y) { continue; }

        potions.push(Potion::new(x, y));
    }

    potions
}
