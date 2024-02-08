use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::time::Duration;
use async_std::task::sleep;
use egui::Context;
use log::{error, log};
use wasm_bindgen_futures::spawn_local;


pub fn list_possible_versions(ctx: &Context, open: &mut bool) {
	spawn_local(async { update_vromf_list().await });
	egui::Window::new("Available versions")
		.open(open)
		.collapsible(false)
		.vscroll(true)
		.show(ctx, |ui| {

			ui.allocate_space(ui.available_size() / 2.0)
		});
}


async fn update_vromf_list() {
	static SPAWNED: AtomicBool = AtomicBool::new(false);
	if SPAWNED.swap(true, Relaxed) {
		return;
	}
	loop {
		sleep(Duration::from_secs(1)).await;
	}
}