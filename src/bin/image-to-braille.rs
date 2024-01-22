use std::io::{stdin, Read};

use clap::Parser;
use funky_subtitles::braille_iter;
use image::imageops::FilterType;

#[derive(Parser)]
struct Args {
	/// The scaling factor for this image.
	#[arg(short, long, default_value = "1.0")]
	scale: f64,
	/// The brightness threshold for this image.
	#[arg(short, long, default_value = "127")]
	threshold: u8,
	/// The color channel to read
	#[arg(short, long, default_value = "0")]
	channel_index: usize,
}

fn main() {
	let Args {
		scale,
		threshold,
		channel_index,
	} = Args::parse();
	let image = {
		let mut data = vec![];
		stdin().lock().read_to_end(&mut data).expect("Read failure");
		data
	};
	let image = image::load_from_memory(&image).expect("Image decode failure");
	let image = {
		let height = (image.height() as f64 * scale) as u32;
		let width = (image.width() as f64 * scale) as u32;
		image.resize(width, height, FilterType::Nearest)
	};
	let image: String = braille_iter(&image, &threshold, channel_index).collect();
	println!("{image}");
}
