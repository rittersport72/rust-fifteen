extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlGraphics, Filter, GlyphCache, TextureSettings, OpenGL};
use piston::input::RenderArgs;
use piston::Button;
use piston::Key;
use rand::prelude::*;

const PIECE_SIZE:f64 = 100.0;
const PIECE_DRAW_SIZE:f64 = PIECE_SIZE - 1.0;
pub static HEIGHT_WIDTH:f64 = PIECE_SIZE * 4.0;
const FONT_SIZE:u32 = 26;

const GRIDX_COUNT: usize = 4;
const GRIDY_COUNT: usize = 4;

const EMPTY_SPACE: u8 = (GRIDX_COUNT * GRIDY_COUNT) as u8;

const FONT: &str = "FiraSans-Regular.ttf";

// The grid[4, 4] numbering
// +----+----+----+----+
// |  1 |  2 |  3 |  4 |
// +----+----+----+----+
// |  5 |  6 |  7 |  8 |
// +----+----+----+----+
// |  9 | 10 | 11 | 12 |
// +----+----+----+----+
// | 13 | 14 | 15 | 16 |
// +----+----+----+----+
pub struct Application {
	gl: GlGraphics,
	grid: [[u8; GRIDX_COUNT]; GRIDY_COUNT],
	fg: [f32; 4],
	bg: [f32; 4],
	tc: [f32; 4],
}

impl Application {
	pub fn new() -> Self {
		let opengl = OpenGL::V3_2;
		let mut app = Application {
			gl: GlGraphics::new(opengl),
			grid: [[0; GRIDX_COUNT]; GRIDY_COUNT],
			fg: [80.0/255.0, 20.0/255.0, 100.0/255.0, 1.0], // purple
			bg: [0.0, 0.0, 0.0, 1.0], // black
			tc: [1.0, 1.0, 1.0, 1.0], // white
		};
		
		for y in 0..GRIDY_COUNT {
			for x in 0..GRIDX_COUNT {
				app.grid[y][x] = (y * GRIDX_COUNT + x + 1) as u8;
				//println!("grid[{}][{}] = {}", y, x, app.grid[y][x]);
			}
		}
		
		app
	}
	
	pub fn render(&mut self, args: &RenderArgs) {
		use graphics::*;
		
		self.gl.draw(args.viewport(), |c, gl| {
			// Clear the screen.
			clear(self.bg ,gl);
			
			let texture_settings = TextureSettings::new().filter(Filter::Nearest);
			let ref mut glyphs = GlyphCache::new(FONT, (), texture_settings).expect(&format!("failed to load font `{}`", FONT));
			
			for y in 0..GRIDY_COUNT {
				for x in 0..GRIDX_COUNT {
					if self.grid[y][x] != EMPTY_SPACE {
						let rect = [x as f64 * PIECE_SIZE, y as f64 * PIECE_SIZE, PIECE_DRAW_SIZE, PIECE_DRAW_SIZE];
						// Draw piece
						rectangle(self.fg, rect, c.transform, gl);
						// Text position for piece
						let transform = c.transform.trans(x as f64 * PIECE_SIZE + 35.0, y as f64 * PIECE_SIZE + 60.0);
						let digit = self.grid[y][x].to_string();
						println!("grid[{}][{}] = {}", y, x, self.grid[y][x]);
						// Draw text
						text(self.tc, FONT_SIZE, &digit[..], glyphs, transform, gl).unwrap();
					}
				}
			}
		});
	}
	
	pub fn press(&mut self, button: &Button) {
		let mut empty_x: i16 = 0;
		let mut empty_y: i16 = 0;
		
		for y in 0..GRIDY_COUNT {
			for x in 0..GRIDX_COUNT {
				if self.grid[y][x] == EMPTY_SPACE {
					empty_x = x as i16;
					empty_y = y as i16;
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
		
		if new_empty_y >= 0 && new_empty_y < GRIDY_COUNT as i16 && new_empty_x >= 0 && new_empty_x < GRIDY_COUNT as i16 {
			self.grid[empty_y as usize][empty_x as usize] = self.grid[new_empty_y as usize][new_empty_x as usize];
			self.grid[new_empty_y as usize][new_empty_x as usize] = EMPTY_SPACE;
		}
	}
	
	pub fn check_complete(&self) {
		let mut complete = true;
		
		for y in 0..GRIDY_COUNT {
			for x in 0..GRIDX_COUNT {
				if self.grid[y][x] != (y * GRIDX_COUNT + x + 1) as u8 {
					complete = false;
					//println!("grid[{}][{}] = {}", y, x, self.grid[y][x]);
				}
			}
		}
		
		if complete {
			println!("complete");
		}
	}
	
	pub fn shuffle(&mut self) {
		let mut rng = thread_rng();
		
		for _i in 0..200 {
			// Exclusive range
			let number: u32 = rng.gen_range(0..4);
			//println!("random number {}", n);
			
			let mut button = Button::Keyboard(Key::Up);
			match number {
				0 => {
					button = Button::Keyboard(Key::Up);
				}
				1 => {
					button = Button::Keyboard(Key::Down);
				}
				2 => {
					button = Button::Keyboard(Key::Left);
				}
				3 => {
					button = Button::Keyboard(Key::Right);
				}
				_ => {
					button = Button::Keyboard(Key::Up);
				}
			}
			
			self.press(&button);
		}
	}
}
