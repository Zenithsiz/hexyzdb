//! Asset build script

#![feature(hash_map_macro)]

mod args;

use {
	self::args::Args,
	app_error::{AppError, Context},
	clap::Parser,
	dhexyzf_build_data::{enemies::Enemies, items::Items, weapons::Weapons},
	std::{fs, hash_map, path::Path},
};

fn main() -> Result<(), AppError> {
	let _logger = zutil_logger::Logger::new();

	let args = Args::parse();

	// TODO: Don't hardcode this relative path, instead receive it as an argument
	let dhexyzf = Path::new("../dhexyzf/");
	let dhexyzf_data = dhexyzf.join("data");

	let images = [
		Image {
			src:  "Title_Tex/4.png",
			dst:  "background.png",
			crop: None,
		},
		Image {
			src:  "Title_Tex/5.png",
			dst:  "logo.png",
			crop: Some(Rect {
				pos:  [0, 0],
				size: [345, 122],
			}),
		},
	];

	for image in images {
		let src = dhexyzf.join(format!("ptx/Hexyz_Force/PSP_GAME/USRDIR/Data/data0/{}", image.src));
		let dst = args.output_dir.join(format!("images/{}", image.dst));
		match image.crop {
			Some(crop) => {
				let image = image::open(src).context("Unable to open image")?;
				let image = image.crop_imm(crop.pos[0], crop.pos[1], crop.size[0], crop.size[1]);
				self::create_parent(&dst)?;
				image.save(dst).context("Unable to save image")?;
			},
			None => self::copy_bin(src, dst)?,
		}
	}

	let weapons = self::read_toml::<Weapons>(dhexyzf_data.join("weapons.toml"))?;
	let enemies = self::read_toml::<Enemies>(dhexyzf_data.join("enemies.toml"))?;
	let items = self::read_toml::<Items>(dhexyzf_data.join("items.toml"))?;

	self::save_json(args.output_dir.join("weapons.json"), &weapons)?;
	self::save_json(args.output_dir.join("enemies.json"), &enemies)?;
	self::save_json(args.output_dir.join("items.json"), &items)?;

	// Weapon/Item icons
	let item_icons = image::open(dhexyzf.join("ptx/Hexyz_Force/PSP_GAME/USRDIR/Data/data0/1stRead/6/7.png"))
		.context("Unable to open item icons")?;

	let elemental_icons = hash_map! {
		// TODO: Decide on a better icon for rare?
		29 => "rare",
		90 => "fire",
		91 => "water",
		92 => "wind",
		93 => "earth",
		94 => "bolt",
		95 => "light",
		96 => "dark",
		104 => "void",
	};

	for icon_idx in 0..132_u32 {
		let icons_per_row = 18;

		let icon_size = 14;

		let x = icon_size * (icon_idx % icons_per_row);
		let y = icon_size * (icon_idx / icons_per_row);

		let icon = item_icons.crop_imm(x, y, icon_size, icon_size);

		let dst = args.output_dir.join(format!("images/icons/items/{icon_idx}.png"));
		self::create_parent(&dst).context("Unable to create parent directory")?;
		icon.save_with_format(dst, image::ImageFormat::Png)
			.context("Unable to save item icon")?;

		if let Some(element) = elemental_icons.get(&icon_idx) {
			let dst = args.output_dir.join(format!("images/icons/elements/{element}.png"));
			self::create_parent(&dst).context("Unable to create parent directory")?;
			icon.save_with_format(dst, image::ImageFormat::Png)
				.context("Unable to save element icon")?;
		}
	}

	for (enemy_idx, enemy) in enemies.enemies.iter().enumerate() {
		// TODO: The actual icon is probably stored inside the model somewhere,
		//       since it doesn't seem to be contained in the enemy.
		#[expect(non_contiguous_range_endpoints, reason = "There are some holes in the models")]
		let icon_idx = match enemy.model {
			// Normal enemies + mimics
			76..183 => enemy.model - 73,

			// Force eater
			183 => 148,

			// Doppelgangers
			184..193 => 110 + enemy.model - 184,

			// Imperial
			193..196 => 119 + enemy.model - 193,

			// Shades
			197..200 => 122 + enemy.model - 197,

			// Levant / Luffina
			223 => 149,
			224 => 150,

			// Phantasma
			200..205 => 125 + enemy.model - 200,

			// Mother worm
			205..207 => 130 + enemy.model - 205,

			// Virtus, Velvet, Faust, Axel, Bahn, Dark Dragon, Dark Elf,
			// Dark Sage, Dark Emperor, Dark General, Azul, Del-Gaia, Nor-Gaia,
			// Force Necrious, Vessel -Creation, Vessel -Destr.
			207..223 => 132 + enemy.model - 207,

			// Galiza
			225..227 => 152,

			// Levant true form
			69 => 202,

			_ => todo!("Unknown model {}", enemy.model),
		};

		self::copy_bin(
			dhexyzf.join(format!(
				"ptx/Hexyz_Force/PSP_GAME/USRDIR/Data/data0/ExtraTex/{icon_idx}.png"
			)),
			args.output_dir.join(format!("images/icons/enemy/{enemy_idx}.png")),
		)?;
	}

	Ok(())
}

struct Image {
	src:  &'static str,
	dst:  &'static str,
	crop: Option<Rect>,
}

struct Rect {
	pos:  [u32; 2],
	size: [u32; 2],
}

fn read_toml<T: serde::de::DeserializeOwned>(path: impl AsRef<Path>) -> Result<T, AppError> {
	let toml = fs::read_to_string(path).context("Unable to read input file")?;
	let value = toml::from_str(&toml).context("Unable to parse input file")?;

	Ok(value)
}

fn save_json<T: serde::Serialize>(path: impl AsRef<Path>, value: &T) -> Result<(), AppError> {
	let path = path.as_ref();
	self::create_parent(path)?;

	let dst_file = fs::File::create(path).context("Unable to create output file")?;
	serde_json::to_writer(dst_file, &value).context("Unable to write output file")?;

	Ok(())
}

fn copy_bin(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<(), AppError> {
	let src = src.as_ref();
	let dst = dst.as_ref();
	self::create_parent(dst)?;

	fs::copy(src, dst).with_context(|| format!("Unable to copy file {} to {}", src.display(), dst.display()))?;
	Ok(())
}

/// Creates the parent directory of `path`, if it exists
fn create_parent(path: &Path) -> Result<(), AppError> {
	if let Some(parent) = path.parent() {
		fs::create_dir_all(parent).context("Unable to create parent directory")?;
	}

	Ok(())
}
