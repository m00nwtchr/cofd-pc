#![feature(iter_array_chunks)]
use std::path::Path;
use std::{fs, path::PathBuf};

use lazy_static::lazy_static;
use reqwest::Url;
use scraper::{ElementRef, Html, Selector};

mod gifts;

lazy_static! {
	static ref PATH: PathBuf =
		Path::new(&std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var"))
			.join("..")
			.join("lib")
			.join("data");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let urls = ["https://codexofdarkness.com/wiki/Gifts"];

	for url in &urls {
		download(url).await?;
	}

	Ok(())
}

async fn download(url: &str) -> anyhow::Result<()> {
	let url = Url::parse(url).expect("Invalid url");

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

	let html_path = path.join(format!("{page_name}.html"));

	let text;
	if !html_path.exists() {
		println!("Downloading: {url}");

		let resp = reqwest::get(url).await?;
		text = resp.text().await?;

		fs::write(html_path, &text)?;
	} else {
		text = fs::read_to_string(html_path)?;
	}

	parse(&page_name, &text)?;

	Ok(())
}

fn parse(page_name: &str, text: &str) -> anyhow::Result<()> {
	let document = Html::parse_document(text);

	let selector = Selector::parse(".mw-parser-output > section, h2").unwrap();
	let table_sel = Selector::parse("table").unwrap();

	let mut vec = Vec::new();
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
			let mut vecc = Vec::new();
			for td in tr.children().filter_map(ElementRef::wrap) {
				vecc.push(td.inner_html().trim().to_string());
			}

			if vecc.len() == 1 {
				vecc.push(title.to_string());
			}
			vec.push(vecc);
		}
	}

	let txt = match page_name {
		"Gifts" => ron::ser::to_string(&gifts::parse_gifts(vec))?,
		_ => String::new(),
	};

	fs::write(PATH.join(format!("{}.ron", page_name.to_lowercase())), txt)?;

	Ok(())
}
