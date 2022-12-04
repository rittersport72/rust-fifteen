use std::path::PathBuf;
use piston_window::*;
use rand::prelude::*;

extern crate piston;

const PIECE_SIZE:f64 = 100.0;
const PIECE_DRAW_SIZE:f64 = PIECE_SIZE - 1.0;
const HEIGHT_WIDTH:f64 = PIECE_SIZE * 4.0;
const FONT_SIZE:u32 = 26;

const GRIDX_COUNT: usize = 5;
const GRIDY_COUNT: usize = 5;

const EMPTY_SPACE: u8 = ((GRIDX_COUNT - 1) * (GRIDY_COUNT - 1)) as u8;


fn shuffle(grid: &mut [[u8; GRIDX_COUNT]; GRIDY_COUNT]) {
	let mut rng = thread_rng();

	for _i in 0..200 {
		// Exclusive range
		let number: u32 = rng.gen_range(1..5);
		//println!("{}", n);

		let mut button = Button::Keyboard(Key::Up);
		match number {
			1 => {
				button = Button::Keyboard(Key::Up);
			}
			2 => {
				button = Button::Keyboard(Key::Down);
			}
			3 => {
				button = Button::Keyboard(Key::Left);
			}
			4 => {
				button = Button::Keyboard(Key::Right);
			}
			_ => {
				button = Button::Keyboard(Key::Up);
			}
		}
	
		press(&button, grid);
	}
}

fn press(button: &Button, grid: &mut [[u8; GRIDX_COUNT]; GRIDY_COUNT]) {
	let mut empty_x = 0;
	let mut empty_y = 0;

	for y in 1..GRIDY_COUNT {
       	for x in 1..GRIDX_COUNT {
       		if grid[y][x] == EMPTY_SPACE {
       			empty_x = x;
       			empty_y = y;
       			println!("empty x {}, y {}", empty_x, empty_y);
       			break;
       		}
       	}
    }
    let mut new_empty_y = empty_y;
	let mut new_empty_x = empty_x;

	if let &Button::Keyboard(key) = button {
        match key {
            Key::Up => {
                new_empty_y = empty_y - 1;
    			println!("Keyboard Up pressed");
            }
            Key::Down => {
                new_empty_y = empty_y + 1;
    			println!("Keyboard Down pressed");
            }
            Key::Left => {
                new_empty_x = empty_x - 1;
    			println!("Keyboard Left pressed");
            }
            Key::Right => {
                new_empty_x = empty_x + 1;
    			println!("Keyboard Right pressed");
            }
            _ => {
            	println!("Keyboard other pressed");
            }
        }
    }

	if new_empty_y > 0 && new_empty_y < GRIDY_COUNT && new_empty_x > 0 && new_empty_x < GRIDY_COUNT {
       	grid[empty_y][empty_x] = grid[new_empty_y][new_empty_x];
       	grid[new_empty_y][new_empty_x] = EMPTY_SPACE;
    }
}

fn check_complete(grid: & [[u8; GRIDX_COUNT]; GRIDY_COUNT]) {
	let mut complete = true;

	for y in 1..GRIDY_COUNT {
       	for x in 1..GRIDX_COUNT {
       		if grid[y][x] != ((y - 1) * (GRIDX_COUNT - 1) + x) as u8 {
       			complete = false;
       			//println!("grid[{}][{}] = {}", y, x, grid[y][x]);
       		}
       	}
    }

    if complete {
    	println!("complete");
    }
}

fn main() {
	let fg = [80.0/255.0, 20.0/255.0, 100.0/255.0, 1.0]; // purple
    let bg = [0.0, 0.0, 0.0, 1.0]; // black
    let tc = [1.0, 1.0, 1.0, 1.0]; // white

    let mut grid = [[0u8; GRIDX_COUNT]; GRIDY_COUNT];

	for y in 1..GRIDY_COUNT {
       	for x in 1..GRIDX_COUNT {
       		grid[y][x] = ((y - 1) * (GRIDX_COUNT - 1) + x) as u8;
       		println!("grid[{}][{}] = {}", y, x, grid[y][x]);
       	}
    }

    shuffle(&mut grid);

	let mut window:PistonWindow = WindowSettings::new("Fifteen", [HEIGHT_WIDTH, HEIGHT_WIDTH])
        .exit_on_esc(true).build().unwrap();

	//let ttf = PathBuf::from(r"/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf");
    let ttf = PathBuf::from(r"FiraSans-Regular.ttf");
    //let ttf = PathBuf::from(r"FiraSans-Regular.ttf");
	let mut glyphs = window.load_font(ttf).unwrap();

	window.set_lazy(true);

	let mut events = window.events;

	while let Some(e) = events.next(&mut window) {
        if let Some(_r) = e.render_args() {
            window.draw_2d( &e, |c, g, device| {
                clear(bg ,g);
                for y in 1..GRIDY_COUNT {
                	for x in 1..GRIDX_COUNT {
                		if grid[y][x] != EMPTY_SPACE {
							rectangle(fg, [(x - 1) as f64 * PIECE_SIZE, (y - 1) as f64 * PIECE_SIZE, PIECE_DRAW_SIZE, PIECE_DRAW_SIZE], c.transform, g);
							let transform = c.transform.trans((x - 1) as f64 * PIECE_SIZE + 35.0, (y - 1) as f64 * PIECE_SIZE + 60.0);
							let digit = grid[y][x].to_string();
							text(tc, FONT_SIZE, &digit[..], &mut glyphs, transform, g).unwrap();
						}
                	}
                }
                // Update glyphs before rendering.
            	glyphs.factory.encoder.flush(device);
            } );
        }
        if let Some(button) = e.press_args() {
        	press(&button, &mut grid);
        	check_complete(&grid);
        }
    }
}
