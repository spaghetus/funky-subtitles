use braille::BRAILLE;
use image::{GenericImageView, Pixel};
use itertools::{iproduct, Itertools};
use std::{fmt::Debug, io::BufReader};

#[allow(clippy::missing_panics_doc)]
pub fn braille_iter<'a, I, P, T>(image: &'a I, threshold: &'a T) -> impl Iterator<Item = char> + 'a
where
	I: GenericImageView<Pixel = P>,
	P: Pixel<Subpixel = T>,
	T: PartialOrd + Clone + Debug,
{
	(0..image.height())
		.tuples()
		.map(|(top, second, third, bottom)| {
			(0..image.width())
				.tuples()
				.map(move |(left, right)| iproduct!([left, right], [top, second, third, bottom]))
		})
		.map(|coordinates| {
			coordinates.map(|block| {
				block
					.map(|(x, y)| image.get_pixel(x, y).channels()[0].clone())
					.map(|value| &value >= threshold)
			})
		})
		.flat_map(|elements| {
			elements
				.map(|flags| -> [usize; 8] {
					flags
						.map(Into::into)
						.collect::<Vec<_>>()
						.try_into()
						.unwrap()
				})
				.map(|[tl, tml, bml, bl, tr, tmr, bmr, br]| {
					BRAILLE[tl][tr][tml][tmr][bml][bmr][bl][br]
				})
				.chain(['\n'])
		})
}

#[test]
fn test_smiley() {
	let smiley = include_bytes!("../test-smiley.png");
	let smiley = image::load_from_memory_with_format(smiley, image::ImageFormat::Png).unwrap();
	let braille = braille_iter(&smiley, &127).collect::<String>();
	println!("{braille}");
}
#[test]
fn test_smiley_non_integer_multiple() {
	let smiley = include_bytes!("../test-smiley-non-integer-multiple.png");
	let smiley = image::load_from_memory_with_format(smiley, image::ImageFormat::Png).unwrap();
	let braille = braille_iter(&smiley, &127).collect::<String>();
	println!("{braille}");
}
