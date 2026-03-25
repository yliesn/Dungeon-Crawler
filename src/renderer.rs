use crossterm::{
    cursor,
    style::{self, Color, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};
use std::io::{self, Write};

use crate::entity::Entity;
use crate::game::{Game, GameState};
use crate::map::Cell;

pub fn render(game: &Game, stdout: &mut impl Write) -> io::Result<()> {
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.queue(cursor::MoveTo(0, 0))?;

    // --- Titre ---
    let title = " ☠  DUNGEON CRAWLER  ☠ ";
    stdout.queue(style::PrintStyledContent(title.with(Color::Yellow).bold()))?;
    stdout.queue(cursor::MoveToNextLine(1))?;

    // --- Carte ---
    for y in 0..game.map.height {
        for x in 0..game.map.width {
            // Joueur ?
            if game.player.x == x && game.player.y == y {
                stdout.queue(style::PrintStyledContent(
                    game.player.symbol().to_string().with(Color::Cyan).bold(),
                ))?;
                continue;
            }

            // Ennemi vivant ?
            if let Some(enemy) = game.enemies.iter().find(|e| e.x == x && e.y == y && e.is_alive()) {
                let color = match enemy.symbol() {
                    'T' => Color::Red,
                    's' => Color::Grey,
                    _   => Color::Green,
                };
                stdout.queue(style::PrintStyledContent(
                    enemy.symbol().to_string().with(color).bold(),
                ))?;
                continue;
            }

            // Potion ?
            if game.potions.iter().any(|p| !p.picked_up && p.x == x && p.y == y) {
                stdout.queue(style::PrintStyledContent(
                    "!".with(Color::Magenta).bold(),
                ))?;
                continue;
            }

            // Case de la carte
            let ch = match game.map.cells[y][x] {
                Cell::Wall  => style::PrintStyledContent("#".with(Color::DarkGrey)),
                Cell::Floor => style::PrintStyledContent(".".with(Color::DarkGrey)),
                Cell::Exit  => style::PrintStyledContent("E".with(Color::Yellow).bold()),
            };
            stdout.queue(ch)?;
        }
        stdout.queue(cursor::MoveToNextLine(1))?;
    }

    // --- HUD joueur ---
    stdout.queue(cursor::MoveToNextLine(1))?;
    let hp_bar = build_hp_bar(game.player.hp, game.player.max_hp, 20);
    stdout.queue(style::PrintStyledContent(
        format!("PV {:>2}/{:<2} {} | Tour: {}", game.player.hp, game.player.max_hp, hp_bar, game.turn)
            .with(Color::White),
    ))?;
    stdout.queue(cursor::MoveToNextLine(1))?;

    // --- Légende ---
    stdout.queue(style::PrintStyledContent(
        "[@] Toi  [g] Goblin  [T] Troll  [s] Squelette  [!] Potion  [E] Sortie"
            .with(Color::DarkGrey),
    ))?;
    stdout.queue(cursor::MoveToNextLine(1))?;
    stdout.queue(style::PrintStyledContent(
        "Contrôles: Z/Q/S/D ou ↑←↓→ | [R] Rejouer | [Echap] Quitter"
            .with(Color::DarkGrey),
    ))?;
    stdout.queue(cursor::MoveToNextLine(1))?;

    // --- Log ---
    stdout.queue(cursor::MoveToNextLine(1))?;
    stdout.queue(style::PrintStyledContent("─── Journal ───".with(Color::DarkYellow)))?;
    stdout.queue(cursor::MoveToNextLine(1))?;

    for line in game.recent_log(5) {
        stdout.queue(style::PrintStyledContent(
            format!(" {}", line).with(Color::White),
        ))?;
        stdout.queue(cursor::MoveToNextLine(1))?;
    }

    // --- Écran de fin ---
    match game.state {
        GameState::Victory => {
            stdout.queue(cursor::MoveToNextLine(1))?;
            stdout.queue(style::PrintStyledContent(
                "🏆  VICTOIRE ! Appuie sur [R] pour rejouer ou [Echap] pour quitter."
                    .with(Color::Yellow).bold(),
            ))?;
        }
        GameState::GameOver => {
            stdout.queue(cursor::MoveToNextLine(1))?;
            stdout.queue(style::PrintStyledContent(
                "☠  GAME OVER. Appuie sur [R] pour rejouer ou [Echap] pour quitter."
                    .with(Color::Red).bold(),
            ))?;
        }
        GameState::Playing => {}
    }

    stdout.flush()?;
    Ok(())
}

fn build_hp_bar(hp: i32, max_hp: i32, width: usize) -> String {
    let hp = hp.max(0);
    let filled = ((hp as f32 / max_hp as f32) * width as f32).round() as usize;
    let filled = filled.min(width);
    let empty = width - filled;
    format!("[{}{}]", "█".repeat(filled), "░".repeat(empty))
}
