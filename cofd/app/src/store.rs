#[cfg(not(target_arch = "wasm32"))]
use directories::ProjectDirs;

#[cfg(not(target_arch = "wasm32"))]
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use cfg_if::cfg_if;

#[cfg(target_arch = "wasm32")]
use anyhow::anyhow;

pub struct Store {
	#[cfg(target_arch = "wasm32")]
	local_storage: web_sys::Storage,
	#[cfg(not(target_arch = "wasm32"))]
	dirs: ProjectDirs,
}

impl Store {
	pub fn new() -> Option<Store> {
		let store;

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
				let dirs = ProjectDirs::from("", "", "cofd-pc").unwrap();

				let dir = dirs.data_dir().parent();
				if dir.is_some() && !dir.unwrap().exists() {
					std::fs::create_dir_all(dir.unwrap()).ok()?;
				}

				store = Some(Self {
					dirs
				});
			}
		}

		store
	}

	pub fn get<T: for<'a> Deserialize<'a>>(&self, name: &str) -> anyhow::Result<Option<T>> {
		let val;

		cfg_if! {
			if #[cfg(target_arch = "wasm32")] {
				val = self.local_storage.get_item(name).map_err(|err| anyhow!("{:?}", err))?;
			} else {
				val = Some(std::fs::read_to_string(
					self.dirs.data_dir().join(format!("{name}.ron")),
				)?);
			}
		}

		if let Some(val) = val {
			Ok(Some(ron::de::from_str(&val)?))
		} else {
			Ok(None)
		}
	}

	pub fn set<T: Serialize>(&self, name: &str, value: &T) -> anyhow::Result<()> {
		let val = ron::ser::to_string(value)?;

		cfg_if! {
			if #[cfg(target_arch = "wasm32")] {
				self.local_storage.set_item(name, &val).map_err(|err| anyhow!("{:?}", err))?;
			} else {
				std::fs::write(self.dirs.data_dir().join(format!("{name}.ron")), val)?;
			}
		}

		Ok(())
	}
}
