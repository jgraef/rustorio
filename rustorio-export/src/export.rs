use std::{
    collections::HashMap,
    fs::File,
    io::{BufWriter, Cursor},
    path::Path,
};

use color_eyre::eyre::Error;
use rustorio_lua_api::loader::Loader;
use rustorio_prototype::{
    achievement::AchievementPrototype,
    item::ItemPrototype,
    recipe::RecipePrototype,
    technology::TechnologyPrototype,
    types::IconSpecification,
    HasPrototypes,
    Prototypes,
};
use image::{io::Reader as ImageReader, ImageFormat};

pub fn export(
    output: impl AsRef<Path>,
    pretty: bool,
    loader: &Loader,
    prototypes: &Prototypes,
) -> Result<(), Error> {
    let output = output.as_ref();
    if !output.exists() {
        std::fs::create_dir_all(output)?;
    }

    let mut icon_data = vec![];
    prototypes.collect_icons(|icon_spec| {
        let (file_name, size, mipmaps) = match icon_spec {
            IconSpecification::Multiple {
                icons,
                icon_size,
                icon_mipmaps,
            } => {
                let Some(icon) = icons.first()
                else {
                    return;
                };
                (&icon.icon, *icon_size, *icon_mipmaps)
            }
            IconSpecification::Single {
                icon,
                icon_size,
                icon_mipmaps,
            } => (icon, Some(*icon_size), *icon_mipmaps),
            IconSpecification::None => return,
        };

        icon_data.push((file_name.clone(), size, mipmaps));
    });

    let mut icons = HashMap::new();
    let icons_output = output.join("icons");
    if !icons_output.exists() {
        std::fs::create_dir(&icons_output)?;
    }
    for (file_name, icon_size, icon_mipmaps) in icon_data {
        let icon_data = loader.read_file(&file_name)?;
        let mut image_reader = ImageReader::new(Cursor::new(icon_data));

        if let Some(ext) = file_name.as_path().extension().and_then(|s| s.to_str()) {
            if let Some(format) = ImageFormat::from_extension(ext) {
                image_reader.set_format(format);
            }
        }
        else {
            image_reader = image_reader.with_guessed_format()?;
        }

        log::debug!("{file_name}: size={icon_size:?}, mipmaps={icon_mipmaps}");

        let image = image_reader.decode()?;
        let image = if let Some(icon_size) = icon_size {
            let icon_size = icon_size as u32;
            image.crop_imm(0, 0, icon_size, icon_size)
        }
        else {
            image
        };

        let output_name = format!("{}.png", icons.len());
        image.save_with_format(icons_output.join(&output_name), ImageFormat::Png)?;
        icons.insert(file_name, output_name);
    }

    let icons_output = BufWriter::new(File::create(output.join("icons.json"))?);
    let prototype_output = BufWriter::new(File::create(output.join("prototypes.json"))?);
    if pretty {
        serde_json::to_writer_pretty(icons_output, &icons)?;
        serde_json::to_writer_pretty(prototype_output, &prototypes)?;
    }
    else {
        serde_json::to_writer(icons_output, &icons)?;
        serde_json::to_writer(prototype_output, &prototypes)?;
    }

    Ok(())
}

trait CollectIcons {
    fn collect_icons<F: FnMut(&IconSpecification)>(&self, f: F);
}

impl CollectIcons for Prototypes {
    fn collect_icons<F: FnMut(&IconSpecification)>(&self, mut f: F) {
        HasPrototypes::<AchievementPrototype>::iter(self).for_each(|p| p.collect_icons(&mut f));
        HasPrototypes::<TechnologyPrototype>::iter(self).for_each(|p| p.collect_icons(&mut f));
        HasPrototypes::<RecipePrototype>::iter(self).for_each(|p| p.collect_icons(&mut f));
        HasPrototypes::<ItemPrototype>::iter(self).for_each(|p| p.collect_icons(&mut f));
    }
}

impl CollectIcons for AchievementPrototype {
    fn collect_icons<F: FnMut(&IconSpecification)>(&self, mut f: F) {
        f(&self.icon_spec);
    }
}

impl CollectIcons for TechnologyPrototype {
    fn collect_icons<F: FnMut(&IconSpecification)>(&self, mut f: F) {
        f(&self.icon_spec);
    }
}

impl CollectIcons for RecipePrototype {
    fn collect_icons<F: FnMut(&IconSpecification)>(&self, mut f: F) {
        f(&self.icon_spec);
    }
}

impl CollectIcons for ItemPrototype {
    fn collect_icons<F: FnMut(&IconSpecification)>(&self, mut f: F) {
        f(&self.icon_spec);
    }
}
