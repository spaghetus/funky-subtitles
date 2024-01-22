use braille::BRAILLE;
use image::{GenericImageView, Pixel};
use itertools::{iproduct, Itertools};

#[allow(clippy::missing_panics_doc)]
pub fn braille_iter<'a, I, P, T>(
	image: &'a I,
	threshold: &'a T,
	channel_index: usize,
) -> impl Iterator<Item = char> + 'a
where
	I: GenericImageView<Pixel = P>,
	P: Pixel<Subpixel = T>,
	T: PartialOrd + Clone,
{
	// Start with the rows of the image
	(0..image.height())
		// Group each set of four rows
		.tuples()
		.map(|(top, second, third, bottom)| {
			// Also, start with the columns of the image
			(0..image.width())
				// And group them into sets of two
				.tuples()
				// Then use the cartesian product to combine them into a grid of eight
				.map(move |(left, right)| iproduct!([left, right], [top, second, third, bottom]))
		})
		// Take those blocks of eight,
		.map(move |coordinates| {
			coordinates.map(move |block| {
				// Read the corresponding pixels from the image,
				block
					.map(move |(x, y)| image.get_pixel(x, y).channels()[channel_index].clone())
					// And replace the coordinates with whether or not that color passed the threshold
					.map(|value| &value >= threshold)
			})
		})
		// Now that we know which pixels should be light and which should be dark,
		// for each row...
		.flat_map(|elements| {
			elements
				// Use those light and dark values to determine a braille character to use
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
	let braille = braille_iter(&smiley, &127, 0).collect::<String>();
	println!("{braille}");
}
#[test]
fn test_smiley_non_integer_multiple() {
	let smiley = include_bytes!("../test-smiley-non-integer-multiple.png");
	let smiley = image::load_from_memory_with_format(smiley, image::ImageFormat::Png).unwrap();
	let braille = braille_iter(&smiley, &127, 0).collect::<String>();
	println!("{braille}");
}
