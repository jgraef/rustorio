use std::{
    collections::HashMap,
    fs::File,
    io::{
        BufWriter,
        Cursor,
    },
    path::Path,
};

use color_eyre::eyre::Error;
use image::{
    io::Reader as ImageReader,
    ImageFormat,
};
use rustorio_lua_api::loader::Loader;
use rustorio_prototype::{
    achievement::AchievementPrototype,
    item::ItemPrototype,
    recipe::RecipePrototype,
    technology::TechnologyPrototype,
    types::{
        FileName,
        IconSpecification,
    },
    HasPrototypes,
    Prototypes,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Data<'a> {
    prototypes: &'a Prototypes,
    icons: HashMap<&'a FileName, String>,
}

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

    let icons = export_icons(output, loader, prototypes)?;

    let output = BufWriter::new(File::create(output.join("data.json"))?);
    let data = Data { prototypes, icons };
    if pretty {
        serde_json::to_writer_pretty(output, &data)?;
    }
    else {
        serde_json::to_writer(output, &data)?;
    }

    Ok(())
}

fn export_icons<'a>(
    output: &Path,
    loader: &Loader,
    prototypes: &'a Prototypes,
) -> Result<HashMap<&'a FileName, String>, Error> {
    let icons_output = output.join("icons");
    if !icons_output.exists() {
        std::fs::create_dir(&icons_output)?;
    }

    let mut icon_data = vec![];
    prototypes.collect_icons(|icon_spec| {
        let (file_name, icon_size, icon_mipmaps) = match icon_spec {
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
        icon_data.push((file_name, icon_size, icon_mipmaps));
    });

    let mut icons = HashMap::new();
    for (file_name, icon_size, icon_mipmaps) in icon_data {
        if !icons.contains_key(file_name) {
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
    }

    Ok(icons)
}

trait CollectIcons {
    fn collect_icons<'a, F: FnMut(&'a IconSpecification)>(&'a self, f: F);
}

impl CollectIcons for Prototypes {
    fn collect_icons<'a, F: FnMut(&'a IconSpecification)>(&'a self, mut f: F) {
        HasPrototypes::<AchievementPrototype>::iter(self).for_each(|p| p.collect_icons(&mut f));
        HasPrototypes::<TechnologyPrototype>::iter(self).for_each(|p| p.collect_icons(&mut f));
        HasPrototypes::<RecipePrototype>::iter(self).for_each(|p| p.collect_icons(&mut f));
        HasPrototypes::<ItemPrototype>::iter(self).for_each(|p| p.collect_icons(&mut f));
    }
}

impl CollectIcons for AchievementPrototype {
    fn collect_icons<'a, F: FnMut(&'a IconSpecification)>(&'a self, mut f: F) {
        f(&self.icon_spec);
    }
}

impl CollectIcons for TechnologyPrototype {
    fn collect_icons<'a, F: FnMut(&'a IconSpecification)>(&'a self, mut f: F) {
        f(&self.icon_spec);
    }
}

impl CollectIcons for RecipePrototype {
    fn collect_icons<'a, F: FnMut(&'a IconSpecification)>(&'a self, mut f: F) {
        f(&self.icon_spec);
    }
}

impl CollectIcons for ItemPrototype {
    fn collect_icons<'a, F: FnMut(&'a IconSpecification)>(&'a self, mut f: F) {
        f(&self.icon_spec);
    }
}
