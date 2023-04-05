#![feature(iter_array_chunks)]
use std::fs;
use std::path::Path;

use convert_case::Casing;
use reqwest::Url;
use ron::ser::PrettyConfig;
use scraper::{ElementRef, Html, Selector};

use cofd_util::scraper::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let urls = ["https://codexofdarkness.com/wiki/Gifts"];

	for url in &urls {
		download(url).await?;
	}

	Ok(())
}

async fn download(url: &str) -> Result<(), Box<dyn std::error::Error>> {
	let url = Url::parse(url).expect("Invalid url");

	let last = url
		.path_segments()
		.expect("Path segments")
		.last()
		.expect("Last path segment")
		.to_string();
	let path = Path::new(&std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR env var"))
		.join("..")
		.join("lib")
		.join("data")
		.join("cache");

	if !path.exists() {
		std::fs::create_dir_all(&path)?;
	}

	let html_path = path.join(format!("{last}.html"));

	let text;
	if !html_path.exists() {
		println!("Downloading: {url}");

		let resp = reqwest::get(url).await?;
		text = resp.text().await?;

		fs::write(html_path, &text)?;
	} else {
		text = fs::read_to_string(html_path)?;
	}

	let document = Html::parse_document(&text);

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

	let mut txt = String::new();
	#[allow(clippy::single_match)]
	match last.as_str() {
		"Gifts" => {
			let mut gift = None;
			let mut gifts = Vec::new();
			for vec in vec {
				if vec.len() == 2 {
					if let Some(g) = gift {
						gifts.push(g);
						gift = None;
					}

					if let Some(name) = vec.first() {
						if !name.contains('(') {
							gift = Some(Gift::new(
								gift_name_to_id(name).to_string(),
								vec.last().unwrap().split(' ').next().unwrap().to_string(),
							));
						}
					}
				} else if let Some(gift) = &mut gift {
					let id = facet_name_to_id(&vec[0]);

					let str = vec[1].clone();

					if str.contains('•') {
						gift.facets.push(Facet::Moon {
							name: id,
							level: str.chars().count() as u16,
						});
					} else {
						gift.facets.push(Facet::Other {
							name: id,
							renown: str,
						});
					}
				}
			}
			if let Some(g) = gift {
				gifts.push(g);
			}

			txt = ron::ser::to_string_pretty(&gifts, PrettyConfig::default())?;
		}
		_ => {}
	}

	fs::write(
		path.join("..").join(format!("{}.ron", last.to_lowercase())),
		txt,
	)?;

	Ok(())
}

fn gift_name_to_id(name: &str) -> &str {
	if name.contains("of") {
		name.split(' ').last().unwrap()
	} else {
		let next = name.split(' ').next().unwrap();
		if next.contains('\'') {
			next.strip_suffix("\'s").unwrap()
		} else {
			next
		}
	}
}

fn facet_name_to_id(name: &str) -> String {
	name.replace('\'', "")
		.replace(',', "")
		.to_case(convert_case::Case::Pascal)
}
