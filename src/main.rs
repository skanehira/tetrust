#![allow(dead_code)]
mod block;
mod game;

use game::{
    draw, erase_line, fix_block, gameover, is_collision, move_block, quit, rotate_left,
    rotate_right, spawn_block, Game, Position,
};
use getch_rs::{Getch, Key};
use std::sync::{Arc, Mutex};
use std::{thread, time};

fn main() {
    let game = Arc::new(Mutex::new(Game::default()));
    println!("\x1b[2J\x1b[H\x1b[?25l");
    draw(&game.lock().unwrap());

    {
        let game = Arc::clone(&game);
        let _ = thread::spawn(move || loop {
            thread::sleep(time::Duration::from_millis(300));
            let mut game = game.lock().unwrap();
            let new_pos = Position {
                x: game.pos.x,
                y: game.pos.y + 1,
            };

            if !is_collision(&game.field, &new_pos, &game.block) {
                game.pos = new_pos;
            } else {
                // ブロックを固定
                fix_block(&mut game);
                // 行削除
                erase_line(&mut game.field);

                if spawn_block(&mut game).is_err() {
                    gameover(&game);
                }
            }
            draw(&game);
        });
    }

    let g = Getch::new();
    loop {
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or_else(|| game.pos.x),
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Char('x')) => {
                let mut game = game.lock().unwrap();
                rotate_right(&mut game);
                draw(&game);
            }
            Ok(Key::Char('z')) => {
                let mut game = game.lock().unwrap();
                rotate_left(&mut game);
                draw(&game);
            }
            Ok(Key::Char('q')) => {
                quit();
            }
            _ => (),
        }
    }
}
