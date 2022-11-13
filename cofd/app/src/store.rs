use std::fs::File;

use cfg_if::cfg_if;

pub struct Store {
	#[cfg(target_arch = "wasm32")]
	local_storage: web_sys::Storage,
	#[cfg(not(target_arch = "wasm32"))]
	file: File,
}

impl Store {
	pub fn new() -> Option<Store> {
		let mut store;

		cfg_if! {
			if #[cfg(target_arch = "wasm32")] {
				let window = web_sys::window()?;
				if let Ok(Some(local_storage)) = window.local_storage() {
					store = Some(Self {
						local_storage,
					});
				} else {
					store = None;
				}
			} else {
				let path = self.save_path();

				let dir = path.parent();
				if dir.is_some() && !dir.unwrap().exists() {
					std::fs::create_dir_all(dir.unwrap()).ok()?;
				}
				let file = File::create(path).ok()?;

				store = Some(Self {
					file
				});
			}
		}



		store
	}

	#[cfg(not(target_arch = "wasm32"))]
	pub fn get(&self, name: &str) -> String {
    	use std::io::Read;

		let mut str = String::new();
		self.file.read_to_string(&mut str);

		str
	}

	#[cfg(target_arch = "wasm32")]
	pub fn get(&self, name: &str) -> String {
		self.local_storage.
	}

	#[cfg(not(target_arch = "wasm32"))]
	fn save_path(&self) -> PathBuf {
		ProjectDirs::from("", "", "cofd-pc")
			.unwrap()
			.data_dir()
			.join("characters.ron")
	}
}
