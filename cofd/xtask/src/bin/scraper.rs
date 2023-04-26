#![feature(iter_array_chunks)]
use std::collections::HashMap;
use std::path::Path;
use std::{fs, path::PathBuf};

use lazy_static::lazy_static;
use reqwest::Url;
use ron::ser::PrettyConfig;
use scraper::{ElementRef, Html, Selector};

mod gifts;
mod merits;

lazy_static! {
	static ref PATH: PathBuf =
		Path::new(&std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var"))
			.join("..")
			.join("lib")
			.join("data");
}

enum PageType {
	Gifts,
	Merits,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let urls = [
		("https://codexofdarkness.com/wiki/Gifts", PageType::Gifts),
		(
			"https://codexofdarkness.com/wiki/Merits,_Universal",
			PageType::Merits,
		),
		// (
		// 	"https://codexofdarkness.com/wiki/Merits,_Vampire",
		// 	PageType::Merits,
		// ),
	];

	let mut handles = Vec::new();
	for (url, page) in urls {
		handles.push(tokio::spawn(download(url.to_string(), page)));
	}

	for handle in handles {
		handle.await.unwrap()?;
	}

	Ok(())
}

async fn download(url: String, page: PageType) -> anyhow::Result<()> {
	let url = Url::parse(&url).expect("Invalid url");

	let page_name = url
		.path_segments()
		.expect("Path segments")
		.last()
		.expect("Last path segment")
		.to_string();

	let path = PATH.join("cache");

	if !path.exists() {
		std::fs::create_dir_all(&path)?;
	}

	let name = page_name.replace(',', "");
	let html_path = path.join(format!("{name}.html"));

	let text;
	if !html_path.exists() {
		println!("Downloading: {url}");

		let resp = reqwest::get(url).await?;
		text = resp.text().await?;

		fs::write(html_path, &text)?;
	} else {
		text = fs::read_to_string(html_path)?;
	}

	parse(&name, &text, page)?;

	Ok(())
}

fn parse(page_name: &str, text: &str, page: PageType) -> anyhow::Result<()> {
	let document = Html::parse_document(text);

	let selector = Selector::parse(".mw-parser-output > section, h2").unwrap();
	let table_sel = Selector::parse("table").unwrap();

	let mut map = HashMap::new();
	for [header, section] in document.select(&selector).skip(1).array_chunks() {
		let title = header.text().last().expect("Last text in header");
		let table = section.select(&table_sel).next().unwrap();

		for tr in table
			.children()
			.last()
			.unwrap()
			.children()
			.filter_map(ElementRef::wrap)
			.skip(1)
		{
			let mut vec = Vec::new();
			for td in tr.children().filter_map(ElementRef::wrap) {
				vec.push(td.inner_html().trim().to_string());
			}

			map.entry(title.to_string()).or_insert(Vec::new()).push(vec);
		}
	}

	let txt = match page {
		PageType::Gifts => ron::ser::to_string(&gifts::parse_gifts(map))?,
		PageType::Merits => {
			ron::ser::to_string_pretty(&merits::parse_merits(map), PrettyConfig::default())?
		} // _ => String::new(),
	};

	fs::write(PATH.join(format!("{}.ron", page_name.to_lowercase())), txt)?;

	Ok(())
}
