mod app;
use app::App;

fn main() {
	let options = eframe::NativeOptions { 
		always_on_top: false, 
		decorated: true, 
		drag_and_drop_support: true, 
		// initial_window_pos: todo!(), 
		// initial_window_size: todo!(), 
		resizable: true, 
		// transparent: todo!(), 
		vsync: true,
		..Default::default()
	};
	
	eframe::run_native("Convi", options, Box::new(|cc| {
		Box::new(App::new(cc))
	}))
}