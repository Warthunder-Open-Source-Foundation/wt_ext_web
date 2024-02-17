use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::OnceLock;
use std::time::Duration;
use async_std::task::sleep;
use egui::Context;
use log::{error, log};
use wasm_bindgen_futures::spawn_local;
use wasm_mt::{Thread, WasmMt};

pub fn list_possible_versions(ctx: &Context, open: &mut bool) {
	spawn_local(async { update_vromf_list().await });

	spawn_local(async {
		let mt = WasmMt::new("./dist/exec.js").and_init().await.unwrap();
		let th = mt.thread().and_init().await.unwrap();
	});

	static COMMITS: OnceLock<Option<Vec<Commit>>> = OnceLock::new();
	 spawn_local(async {
		 return;
		 if COMMITS.get().is_some() {
			 return;
		 }
		let t = reqwest::get("https://api.github.com/repos/gszabi99/War-Thunder-Datamine/commits")
			.await.
			unwrap()
			.text()
			.await
			.unwrap();
		 COMMITS.set(Some(serde_json::from_str(&t).unwrap())).unwrap();
	});

	egui::Window::new("Available versions")
		.open(open)
		.collapsible(false)
		.vscroll(true)
		.show(ctx, |ui| {
			error!("{:?}", COMMITS.get());
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


// Git commits API structure

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Commit {
	sha: String,
	message: String,
}