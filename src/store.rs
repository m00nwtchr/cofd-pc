use serde::{Deserialize, Serialize};

pub struct Store {
	#[cfg(target_arch = "wasm32")]
	local_storage: web_sys::Storage,
	#[cfg(not(target_arch = "wasm32"))]
	dirs: directories::ProjectDirs,
}

impl Store {
	pub fn new() -> Option<Store> {
		#[cfg(target_arch = "wasm32")]
		{
			let window = web_sys::window()?;
			if let Ok(Some(local_storage)) = window.local_storage() {
				Some(Self { local_storage })
			} else {
				None
			}
		}
		#[cfg(not(target_arch = "wasm32"))]
		{
			let dirs = directories::ProjectDirs::from("", "", "cofd-pc").unwrap();

			let dir = dirs.data_dir().parent();
			if dir.is_some() && !dir.unwrap().exists() {
				std::fs::create_dir_all(dir.unwrap()).ok()?;
			}

			Some(Self { dirs })
		}
	}

	pub fn get<T: for<'a> Deserialize<'a>>(&self, name: &str) -> anyhow::Result<Option<T>> {
		#[cfg(target_arch = "wasm32")]
		let val = self
			.local_storage
			.get_item(name)
			.map_err(|err| anyhow::anyhow!("{:?}", err))?;

		#[cfg(not(target_arch = "wasm32"))]
		let val = Some(std::fs::read_to_string(
			self.dirs.data_dir().join(format!("{name}.ron")),
		)?);

		if let Some(val) = val {
			Ok(Some(ron::de::from_str(&val)?))
		} else {
			Ok(None)
		}
	}

	pub fn set<T: Serialize>(&self, name: &str, value: &T) -> anyhow::Result<()> {
		let val = ron::ser::to_string(value)?;

		#[cfg(target_arch = "wasm32")]
		self.local_storage
			.set_item(name, &val)
			.map_err(|err| anyhow::anyhow!("{:?}", err))?;

		#[cfg(not(target_arch = "wasm32"))]
		std::fs::write(self.dirs.data_dir().join(format!("{name}.ron")), val)?;

		Ok(())
	}
}
