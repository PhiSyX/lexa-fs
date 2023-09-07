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

use std::{fmt, str};

// ----------- //
// Énumération //
// ----------- //

/// Extensions de fichiers supportées par les fonctions [load()] et
/// [load_or_prompt()].
#[derive(Debug)]
pub enum Extension {
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

impl str::FromStr for Extension {
	type Err = &'static str;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		Ok(match s {
			| "" | "local" | "development" | "test" => Self::ENV,
			| "json" => Self::JSON,
			| "toml" => Self::TOML,
			| "yml" | "yaml" => Self::YAML,
			| _ => {
				return Err("Extension de fichier dé-sérialisable non valide")
			}
		})
	}
}

impl fmt::Display for Extension {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let extension = match self {
			| Extension::ENV => "env",
			| Extension::JSON => "json",
			| Extension::TOML => "toml",
			| Extension::YAML => "yml",
		};
		write!(f, "{}", extension)
	}
}
