// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::{fmt, str};

// ----------- //
// Énumération //
// ----------- //

#[derive(Debug)]
#[derive(Copy, Clone)]
#[derive(PartialEq, Eq)]
/// Extensions de fichiers supportées par les fonctions [crate::load()] et
/// [crate::load_or_prompt()].
pub enum Extension
{
	/// Correspond aux extensions de fichiers `.local`, `.development`, `.test`
	ENV,
	/// Correspond à l'extension de fichier `.json`
	JSON,
	/// Correspond à l'extension de fichier `.toml`
	TOML,
	/// Correspond aux extensions de fichiers `.yml`, `.yaml`
	YAML,
}

// -------------- //
// Implémentation // -> Interface
// -------------- //

impl str::FromStr for Extension
{
	type Err = String;

	fn from_str(extension: &str) -> Result<Self, Self::Err>
	{
		Ok(match extension {
			// NOTE: fichier .env (.env.local, .env.development, .env.test)
			| "" | "local" | "development" | "test" => Self::ENV,
			| "json" => Self::JSON,
			| "toml" => Self::TOML,
			| "yml" | "yaml" => Self::YAML,
			| _ => return Err(format!("L'extension de fichier « {extension} » n'est pas valide")),
		})
	}
}

impl fmt::Display for Extension
{
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
	{
		let extension = match self {
			| Extension::ENV => "env",
			| Extension::JSON => "json",
			| Extension::TOML => "toml",
			| Extension::YAML => "yml",
		};
		write!(f, "{}", extension)
	}
}
