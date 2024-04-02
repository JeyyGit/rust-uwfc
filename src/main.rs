use clap::{arg, value_parser, Command};
use rand::prelude::SliceRandom;
use std::collections::{HashMap, HashSet};
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
struct Sides {
    up: u8,
    right: u8,
    down: u8,
    left: u8,
}

#[derive(Debug, Clone)]
struct Tile {
    tile: char,
    sides: Sides,
    entropy: usize,
    possibilities: HashMap<String, HashSet<char>>,
    allowed: Vec<Tile>,
    active: bool,
}

impl Tile {
    fn new(tile: char, sides: (u8, u8, u8, u8)) -> Tile {
        Tile {
            tile: tile,
            sides: Sides {
                up: sides.0,
                right: sides.1,
                down: sides.2,
                left: sides.3,
            },
            entropy: 13,
            possibilities: Tile::init_possibilities(),
            allowed: Vec::new(),
            active: false,
        }
    }

    fn init_possibilities() -> HashMap<String, HashSet<char>> {
        let mut possibilities: HashMap<String, HashSet<char>> = HashMap::new();
        possibilities.insert(String::from("up"), HashSet::new());
        possibilities.insert(String::from("right"), HashSet::new());
        possibilities.insert(String::from("down"), HashSet::new());
        possibilities.insert(String::from("left"), HashSet::new());

        possibilities
    }

    fn update_entropy(&mut self, tiles: &[Tile]) {
        let values: Vec<&HashSet<char>> = self
            .possibilities
            .values()
            .filter(|val| !val.is_empty())
            .collect();

        if !values.is_empty() {
            let mut allowed = values[0].clone();
            for other in &values[1..] {
                allowed = allowed.intersection(other).cloned().collect();
            }

            self.allowed = allowed
                .iter()
                .filter_map(|&ch| Tile::get_tile_from_char(&tiles, ch))
                .cloned()
                .collect();

            self.entropy = self.allowed.len();
        } else {
            self.allowed = Vec::new();
            self.entropy = 99;
        }
    }

    fn get_tile_from_char(tiles: &[Tile], tile_char: char) -> Option<&Tile> {
        tiles.iter().find(|&tile| tile.tile == tile_char)
    }
}

fn print_board(board: &Vec<Vec<Tile>>) {
    for row in board {
        for tile in row {
            if tile.active {
                print!("#");
            } else if tile.tile == '-' {
                print!("*");
            } else {
                print!("{}", tile.tile);
            }
        }
        println!();
    }
    println!("\n");
}

fn update_entropies(board: &mut Vec<Vec<Tile>>, tiles: &[Tile]) {
    let h = board.len();
    let w = board[0].len();
    let tile_set: HashSet<char> = tiles.iter().map(|tile| tile.tile).collect();

    for i in 0..h {
        for j in 0..w {
            let mut tile = board[i][j].clone();

            for poss in tile.possibilities.values_mut() {
                poss.clear();
            }

            if !tile.active {
                continue;
            }
            if board[i][j].tile == '-' {
                if i > 0 {
                    let up = &board[i - 1][j];

                    if up.tile != '-' {
                        for tl in tiles {
                            if tl.sides.up == up.sides.down {
                                tile.possibilities
                                    .get_mut(&String::from("up"))
                                    .unwrap()
                                    .insert(tl.tile);
                            }
                        }
                    } else {
                        tile.possibilities
                            .get_mut(&String::from("up"))
                            .unwrap()
                            .extend(tile_set.clone());
                    }
                }
                if j < w - 1 {
                    let right = &board[i][j + 1];

                    if right.tile != '-' {
                        for tl in tiles {
                            if tl.sides.right == right.sides.left {
                                tile.possibilities
                                    .get_mut(&String::from("right"))
                                    .unwrap()
                                    .insert(tl.tile);
                            }
                        }
                    } else {
                        tile.possibilities
                            .get_mut(&String::from("right"))
                            .unwrap()
                            .extend(tile_set.clone());
                    }
                }
                if i < h - 1 {
                    let down = &board[i + 1][j];

                    if down.tile != '-' {
                        for tl in tiles {
                            if tl.sides.down == down.sides.up {
                                tile.possibilities
                                    .get_mut(&String::from("down"))
                                    .unwrap()
                                    .insert(tl.tile);
                            }
                        }
                    } else {
                        tile.possibilities
                            .get_mut(&String::from("down"))
                            .unwrap()
                            .extend(tile_set.clone());
                    }
                }
                if j > 0 {
                    let left = &board[i][j - 1];

                    if left.tile != '-' {
                        for tl in tiles {
                            if tl.sides.left == left.sides.right {
                                tile.possibilities
                                    .get_mut(&String::from("left"))
                                    .unwrap()
                                    .insert(tl.tile);
                            }
                        }
                    } else {
                        tile.possibilities
                            .get_mut(&String::from("left"))
                            .unwrap()
                            .extend(tile_set.clone());
                    }
                }
            }
            board[i][j] = tile;
            board[i][j].update_entropy(tiles);
        }
    }
}

fn find_random_lowest_entropy_index(board: &[Vec<Tile>]) -> (usize, usize) {
    let lowest_entropy = board
        .iter()
        .flat_map(|row| row.iter())
        .map(|tile| tile.entropy)
        .min()
        .unwrap();

    let mut lowest_entropy_tiles = Vec::new();
    for (i, row) in board.iter().enumerate() {
        for (j, tile) in row.iter().enumerate() {
            if tile.entropy == lowest_entropy {
                lowest_entropy_tiles.push((i, j));
            }
        }
    }

    lowest_entropy_tiles
        .choose(&mut rand::thread_rng())
        .cloned()
        .unwrap()
}

fn update_adjacent_tiles(board: &mut Vec<Vec<Tile>>, x: usize, y: usize) {
    let width = board.len();
    let height = board[0].len();
    board[x][y].active = false;

    if x > 0 {
        let tile_above = &mut board[x - 1][y];
        if tile_above.tile == '-' {
            tile_above.active = true;
        }
    }

    if y < height - 1 {
        let tile_right = &mut board[x][y + 1];
        if tile_right.tile == '-' {
            tile_right.active = true;
        }
    }

    if x < width - 1 {
        let tile_below = &mut board[x + 1][y];
        if tile_below.tile == '-' {
            tile_below.active = true;
        }
    }

    if y > 0 {
        let tile_left = &mut board[x][y - 1];
        if tile_left.tile == '-' {
            tile_left.active = true;
        }
    }
}

fn parse_args() -> (usize, usize, usize) {
    let matches = Command::new("MyApp")
        .version("1.0")
        .about("Does awesome things")
        .arg(
            arg!(--height <VALUE>)
                .help("height of generated pattern")
                .default_value("10")
                .value_parser(value_parser!(u32).range(1..)),
        )
        .arg(
            arg!(--width <VALUE>)
                .help("width of generated pattern")
                .default_value("10")
                .value_parser(value_parser!(u32).range(1..)),
        )
        .arg(
            arg!(--"n-iter" <VALUE>)
                .help("how many patterns generates")
                .default_value("1")
                .value_parser(value_parser!(u32).range(1..)),
        )
        .get_matches();

    let width = matches.get_one::<u32>("width").expect("required").clone() as usize;
    let height = matches.get_one::<u32>("height").expect("required").clone() as usize;
    let n_iter = matches.get_one::<u32>("n-iter").expect("required").clone() as usize;

    (width, height, n_iter)
}

fn main() {
    let (width, height, n_iter) = parse_args();

    let tiles = [
        Tile::new(' ', (0, 0, 0, 0)),
        Tile::new('╠', (1, 1, 1, 0)),
        Tile::new('╦', (0, 1, 1, 1)),
        Tile::new('╣', (1, 0, 1, 1)),
        Tile::new('╩', (1, 1, 0, 1)),
        Tile::new('╔', (0, 1, 1, 0)),
        Tile::new('╗', (0, 0, 1, 1)),
        Tile::new('╚', (1, 1, 0, 0)),
        Tile::new('╝', (1, 0, 0, 1)),
        Tile::new('╬', (1, 1, 1, 1)),
        Tile::new('║', (1, 0, 1, 0)),
        Tile::new('═', (0, 1, 0, 1)),
        Tile::new('├', (2, 2, 2, 0)),
        Tile::new('┬', (0, 2, 2, 2)),
        Tile::new('┤', (2, 0, 2, 2)),
        Tile::new('┴', (2, 2, 0, 2)),
        Tile::new('┌', (0, 2, 2, 0)),
        Tile::new('┐', (0, 0, 2, 2)),
        Tile::new('└', (2, 2, 0, 0)),
        Tile::new('┘', (2, 0, 0, 2)),
        Tile::new('┼', (2, 2, 2, 2)),
        Tile::new('│', (2, 0, 2, 0)),
        Tile::new('─', (0, 2, 0, 2)),
        Tile::new('╨', (1, 2, 0, 2)),
        Tile::new('╡', (2, 0, 2, 1)),
        Tile::new('╥', (0, 2, 1, 2)),
        Tile::new('╞', (2, 1, 2, 0)),
    ];

    // SET SIZE UNICODE WAVE FUNCTION COLLAPSE SHAPE
    let shape = (height, width);

    for iter in 0..n_iter {
        let mut board = vec![vec![Tile::new('-', (0, 0, 0, 0)); shape.1]; shape.0];

        let time = Instant::now();
        for i in 0..shape.0 * shape.1 {
            if i != 0 {
                update_entropies(&mut board, &tiles);

                let (rand_x, rand_y) = find_random_lowest_entropy_index(&board);
                let chosen_tile = board[rand_x][rand_y]
                    .allowed
                    .choose(&mut rand::thread_rng())
                    .unwrap()
                    .clone();
                board[rand_x][rand_y] = chosen_tile.clone();

                println!("chosen: {}, at: {:?}", chosen_tile.tile, (rand_x, rand_y));
                update_adjacent_tiles(&mut board, rand_x, rand_y);

                print_board(&board);
            } else {
                let (rand_x, rand_y) = find_random_lowest_entropy_index(&board);
                board[rand_x][rand_y] = tiles.choose(&mut rand::thread_rng()).unwrap().clone();
                update_adjacent_tiles(&mut board, rand_x, rand_y);
                print_board(&board);
            }
        }

        println!(
            "UWFC size of {:?} took {} seconds",
            shape,
            time.elapsed().as_secs_f32()
        );
        if iter < n_iter - 1 {
            thread::sleep(Duration::from_secs(3));
        }
    }
}
