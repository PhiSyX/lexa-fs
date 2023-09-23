// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::io::Write;
use std::{fmt, io, path};

use console::style;

use crate::Extension;

// -------- //
// Fonction //
// -------- //

/// Charge un fichier à partir d'un dossier, et le dé-sérialise en un type donné
/// par générique. Si le fichier n'est pas présent, l'utilisateur sera
/// questionné sur les valeurs à remplir, par rapport aux champs de la structure
/// donné, pour le créer.
pub fn load_or_prompt<T>(
	directory: impl AsRef<path::Path>,
	filename: impl fmt::Display,
	extension: impl Into<Extension>,
) -> io::Result<T>
where
	T: fmt::Debug,
	T: serde::Serialize + serde::de::DeserializeOwned,
	T: lexa_prompt::Prompt,
{
	let extension = extension.into();
	let fullpath = path::Path::new(directory.as_ref())
		.join(format!("{filename}.{extension}"));

	if !fullpath.exists() {
		return save(fullpath, extension);
	}

	crate::load(directory, filename, &extension)
		.or_else(|_| save(fullpath, extension))
}

fn save<T>(
	fullpath: impl AsRef<path::Path>,
	extension: impl Into<Extension>,
) -> io::Result<T>
where
	T: fmt::Debug,
	T: serde::Serialize + serde::de::DeserializeOwned,
	T: lexa_prompt::Prompt,
{
	log::warn!(
		"Les données du fichier « {} » sont manquantes ou corrompues.",
		style(fullpath.as_ref().display()).yellow()
	);

	println!();
	println!(
		"Création du formulaire pour le fichier « {} »...",
		style(fullpath.as_ref().display()).yellow()
	);
	println!();

	let data = T::prompt().map_err(|err| {
		io::Error::new(io::ErrorKind::InvalidData, err.to_string())
	})?;

	log::debug!("  Les données du formulaire sont\n{:#?}", &data);

	if !lexa_prompt::confirm(format!(
		"Sauvegarder le résultat dans « {} »",
		style(fullpath.as_ref().display()).yellow()
	)) {
		return Ok(data);
	}

	let extension = extension.into();

	let mut fd = std::fs::File::create(fullpath)?;

	let content = match extension.to_string().parse() {
		| Ok(Extension::JSON) => {
			serde_json::to_string_pretty(&data)
				.map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
		}

		| Ok(Extension::TOML) => {
			serde_toml::to_string_pretty(&data)
				.map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
		}

		| Ok(Extension::YAML) => {
			serde_yaml::to_string(&data)
				.map_err(|err| io::Error::new(io::ErrorKind::Other, err))?
		}

		| _ => unimplemented!(),
	};

	write!(fd, "{}", content)?;

	Ok(data)
}
