use crate::block::{BlockKind, BlockShape, BLOCKS};

pub const FIELD_WIDTH: usize = 11 + 2 + 1;
pub const FIELD_HEIGHT: usize = 20 + 1 + 1;
pub type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Default for Position {
    fn default() -> Self {
        Position { x: 5, y: 0 }
    }
}

pub struct Game {
    pub field: Field,
    pub pos: Position,
    pub block: BlockShape,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            field: [
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0],
                [0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            pos: Position::default(),
            block: BLOCKS[rand::random::<BlockKind>() as usize],
        }
    }
}

pub fn is_collision(field: &Field, pos: &Position, block: &BlockShape) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            if y + pos.y >= FIELD_HEIGHT - 1 || x + pos.x >= FIELD_WIDTH - 1 {
                continue;
            }
            if field[y + pos.y][x + pos.x] & block[y][x] == 1 {
                return true;
            }
        }
    }
    false
}

#[allow(clippy::needless_range_loop)]
pub fn draw(Game { field, pos, block }: &Game) {
    let mut field_buf = *field;

    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] == 1 {
                field_buf[y + pos.y][x + pos.x] = 1;
            }
        }
    }

    println!("\x1b[H");
    for y in 0..FIELD_HEIGHT - 1 {
        for x in 0..FIELD_WIDTH - 1 {
            if field_buf[y][x] == 1 {
                print!("[]");
            } else {
                print!("  ");
            }
        }
        println!();
    }
}

pub fn fix_block(Game { field, pos, block }: &mut Game) {
    for y in 0..4 {
        for x in 0..4 {
            if block[y][x] == 1 {
                field[y + pos.y][x + pos.x] = 1;
            }
        }
    }
}

pub fn erase_line(field: &mut Field) {
    for y in 1..FIELD_HEIGHT - 2 {
        let mut can_erase = true;
        for x in 1..FIELD_WIDTH - 1 {
            if field[y][x] == 0 {
                can_erase = false;
                break;
            }
        }
        if can_erase {
            for y2 in (2..=y).rev() {
                field[y2] = field[y2 - 1];
            }
        }
    }
}

pub fn move_block(game: &mut Game, new_pos: Position) {
    if !is_collision(&game.field, &new_pos, &game.block) {
        game.pos = new_pos;
    }
}

pub fn spawn_block(game: &mut Game) -> Result<(), ()> {
    game.pos = Position::default();
    game.block = BLOCKS[rand::random::<BlockKind>() as usize];

    if is_collision(&game.field, &game.pos, &game.block) {
        Err(())
    } else {
        Ok(())
    }
}

pub fn gameover(game: &Game) -> ! {
    draw(game);
    println!("GAMEOVER");
    quit();
}

pub fn quit() -> ! {
    println!("\x1b[?25h");
    std::process::exit(0);
}

pub fn rotate_right(game: &mut Game) {
    let mut new_shape: BlockShape = Default::default();
    for y in 0..4 {
        for x in 0..4 {
            new_shape[y][x] = game.block[4 - 1 - x][y];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.block = new_shape;
    }
}
pub fn rotate_left(game: &mut Game) {
    let mut new_shape: BlockShape = Default::default();
    for y in 0..4 {
        for x in 0..4 {
            new_shape[4 - 1 - x][y] = game.block[y][x];
        }
    }
    if !is_collision(&game.field, &game.pos, &new_shape) {
        game.block = new_shape;
    }
}
