extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;
use piston::PressEvent;
use piston::event_loop::{EventSettings, Events};
use piston::input::RenderEvent;
use piston::window::WindowSettings;

pub mod fifteen;


fn main() {
	// Change this to OpenGL::V2_1 if not working.
	let opengl = OpenGL::V3_2;
	
	// Create a Glutin window.
	let mut window: Window = WindowSettings::new("fifteen", [crate::fifteen::HEIGHT_WIDTH, crate::fifteen::HEIGHT_WIDTH])
	.graphics_api(opengl)
	.exit_on_esc(true)
	.build()
	.unwrap();
	
	// Create a new game and run it.
	let mut app = fifteen::Application::new();
	//app.shuffle();
	
	let mut events = Events::new(EventSettings::new());
	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			app.render(&args);
		}
		
		if let Some(button) = e.press_args() {
			app.press(&button);
			app.check_complete();
		}
	}
}
