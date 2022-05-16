#![allow(non_upper_case_globals)]
use dioxus::prelude::*;

fn main() {
	dioxus::desktop::launch_cfg(app, |cfg| {
		cfg.with_window(|window| {
			window.with_decorations(true)
				.with_transparent(true)
				.with_title("Convi")
		})
	});
}

fn app(cx: Scope) -> Element {
	let n = ["a", "b", "c", "d", "e"].iter().map(|e| rsx!(
		li { "Name: {e}" }
	));
	cx.render(rsx!(
		body {
			div {
				h1 { "Dioxus™" }
				p { "A web-based desktop-native GUI library for Rust." }
				ul { n }
			}
		}
	))
}