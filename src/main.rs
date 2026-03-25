mod map;
mod entity;
mod potion;
mod game;
mod renderer;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEventKind},
    terminal, ExecutableCommand,
};
use std::io;
use game::{Game, GameState};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    // Mode raw : lecture touche par touche, sans Entrée
    terminal::enable_raw_mode()?;
    stdout.execute(cursor::Hide)?;

    let mut game = Game::new();

    loop {
        renderer::render(&game, &mut stdout)?;

        // Lecture d'un événement clavier
        if let Event::Key(key) = event::read()? {
            // Ignorer les KeyRelease (Windows)
            if key.kind == KeyEventKind::Release {
                continue;
            }

            match key.code {
                // Quitter
                KeyCode::Esc => break,

                // Rejouer
                KeyCode::Char('r') | KeyCode::Char('R') => {
                    game = Game::new();
                    continue;
                }

                // Mouvement — seulement si la partie est en cours
                _ if game.state == GameState::Playing => {
                    let (dx, dy) = match key.code {
                        KeyCode::Char('z') | KeyCode::Char('Z') | KeyCode::Up    => (0, -1),
                        KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down  => (0, 1),
                        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Left  => (-1, 0),
                        KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => (1, 0),
                        _ => continue,
                    };
                    game.move_player(dx, dy);
                }

                _ => {}
            }
        }
    }

    // Nettoyage terminal
    terminal::disable_raw_mode()?;
    stdout.execute(cursor::Show)?;
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    stdout.execute(cursor::MoveTo(0, 0))?;

    println!("À bientôt dans le donjon !");
    Ok(())
}
