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

const GRIDX_COUNT: usize = 5;
const GRIDY_COUNT: usize = 5;

const EMPTY_SPACE: u8 = ((GRIDX_COUNT - 1) * (GRIDY_COUNT - 1)) as u8;

const FONT: &str = "FiraSans-Regular.ttf";

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
		
		for y in 1..GRIDY_COUNT {
			for x in 1..GRIDX_COUNT {
				app.grid[y][x] = ((y - 1) * (GRIDX_COUNT - 1) + x) as u8;
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
			
			
			for y in 1..GRIDY_COUNT {
				for x in 1..GRIDX_COUNT {
					if self.grid[y][x] != EMPTY_SPACE {
						let rect = [(x - 1) as f64 * PIECE_SIZE, (y - 1) as f64 * PIECE_SIZE, PIECE_DRAW_SIZE, PIECE_DRAW_SIZE];
						rectangle(self.fg, rect, c.transform, gl);
						
						let transform = c.transform.trans((x - 1) as f64 * PIECE_SIZE + 35.0, (y - 1) as f64 * PIECE_SIZE + 60.0);
						let digit = self.grid[y][x].to_string();
						text(self.tc, FONT_SIZE, &digit[..], glyphs, transform, gl).unwrap();
					}
				}
			}
		});
	}
	
	pub fn press(&mut self, button: &Button) {
		let mut empty_x = 0;
		let mut empty_y = 0;
		
		for y in 1..GRIDY_COUNT {
			for x in 1..GRIDX_COUNT {
				if self.grid[y][x] == EMPTY_SPACE {
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
			self.grid[empty_y][empty_x] = self.grid[new_empty_y][new_empty_x];
			self.grid[new_empty_y][new_empty_x] = EMPTY_SPACE;
		}
	}
	
	pub fn check_complete(&self) {
		let mut complete = true;
		
		for y in 1..GRIDY_COUNT {
			for x in 1..GRIDX_COUNT {
				if self.grid[y][x] != ((y - 1) * (GRIDX_COUNT - 1) + x) as u8 {
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
			
			self.press(&button);
		}
	}
}
