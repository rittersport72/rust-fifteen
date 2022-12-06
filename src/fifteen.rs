
pub struct App {
	gl: GlGraphics,
	grid: [[u8; GRIDX_COUNT]; GRIDY_COUNT],
	fg: [f32; 4],
	bg: [f32; 4],
	tc: [f32; 4],
}

impl App {
	pub fn new() -> Self {
        App {
            grid: [[0; GRIDX_COUNT]; GRIDY_COUNT],
			fg: [80.0/255.0, 20.0/255.0, 100.0/255.0, 1.0], // purple
			bg: [0.0, 0.0, 0.0, 1.0], // black
    		tc: [1.0, 1.0, 1.0, 1.0], // white	
        }
    }

    fn render(&mut self, args: &RenderArgs) {
		use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square = rectangle::square(0.0, 0.0, 50.0);
        let rotation = self.rotation;
        let (x, y) = (args.window_size[0] / 2.0, args.window_size[1] / 2.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x, y)
                .rot_rad(rotation)
                .trans(-25.0, -25.0);

            // Draw a box rotating around the middle of the screen.
            rectangle(RED, square, transform, gl);
        });
    }

	fn press(&mut self, button: &Button) {
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

	fn check_complete(&self) {
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
}
