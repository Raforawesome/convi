mod app;
use app::App;
use eframe::emath::Vec2;

fn main() {
	let options = eframe::NativeOptions { 
		always_on_top: false, 
		decorated: true, 
		drag_and_drop_support: true, 
		// initial_window_pos: Some(500), 
		initial_window_size: Some(Vec2::new(500_f32, 500_f32)), 
		resizable: true, 
		// transparent: todo!(), 
		vsync: true,
		..Default::default()
	};
	
	eframe::run_native("Convi", options, Box::new(|cc| {
		Box::new(App::new(cc))
	}))
}