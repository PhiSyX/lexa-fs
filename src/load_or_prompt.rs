// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃                           __    _            ____  ______                 ┃
// ┃                    ____  / /_  (_)______  __/ __ \/ ____/                 ┃
// ┃                   / __ \/ __ \/ / ___/ / / / /_/ / /                      ┃
// ┃                  / /_/ / / / / (__  ) /_/ / _, _/ /___                    ┃
// ┃                 / .___/_/ /_/_/____/\__, /_/ |_|\____/                    ┃
// ┃                /_/                 /____/                                 ┃
// ┣━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┫
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
	extension: impl fmt::Display,
) -> io::Result<T>
where
	T: fmt::Debug,
	T: serde::Serialize + serde::de::DeserializeOwned,
	T: lexa_prompt::Prompt,
{
	let fullpath = path::Path::new(directory.as_ref())
		.join(format!("{filename}.{extension}"));

	if !fullpath.exists() {
		return save(extension, fullpath);
	}

	crate::load(directory, filename, &extension)
		.or_else(|_| save(extension, fullpath))
}

fn save<T>(
	extension: impl fmt::Display,
	fullpath: impl AsRef<path::Path>,
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

	let data = T::prompt()?;

	log::debug!("  Les données du formulaire sont\n{:#?}", &data);

	if lexa_prompt::confirm(format!(
		"Sauvegarder le résultat dans « {} »",
		style(fullpath.as_ref().display()).yellow()
	)) {
		let mut fd = std::fs::File::create(fullpath)?;
		let content = match extension.to_string().parse::<Extension>() {
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
	}

	Ok(data)
}
