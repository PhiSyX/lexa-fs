// ┏━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┓
// ┃ Copyright: (c) 2023, Mike 'PhiSyX' S. (https://github.com/PhiSyX)         ┃
// ┃ SPDX-License-Identifier: MPL-2.0                                          ┃
// ┃ ╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌ ┃
// ┃                                                                           ┃
// ┃  This Source Code Form is subject to the terms of the Mozilla Public      ┃
// ┃  License, v. 2.0. If a copy of the MPL was not distributed with this      ┃
// ┃  file, You can obtain one at https://mozilla.org/MPL/2.0/.                ┃
// ┗━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━┛

use std::{fs, io, path};

// -------- //
// Fonction //
// -------- //

/// Copie un répertoire vers une destination.
pub fn copy_dir(dir: impl AsRef<path::Path>, dest: impl AsRef<path::Path>) -> io::Result<()>
{
	let dir = dir.as_ref();

	if !dir.is_dir() {
		let err = format!(
			"L'argument `dir` « {} » ne semble pas être un répertoire.",
			dir.display(),
		);
		// FIXME: utiliser la variante [io::ErrorKind::NotADirectory] lorsque ce
		// sera stable.
		return Err(io::Error::new(io::ErrorKind::Other, err));
	}

	let dest = dest.as_ref();

	if !dest.is_dir() {
		fs::create_dir(dest)?;
		log::trace!("Le répertoire « {} » a été crée.", dest.display());
	}

	for from in fs::read_dir(dir)? {
		let dir_entry  = from?;
		let file_type = dir_entry.file_type()?;

		if file_type.is_dir() {
			copy_dir(dir_entry.path(), dest.join(dir_entry.file_name()))?;
		} else {
			fs::copy(dir_entry.path(), dest.join(dir_entry.file_name()))?;
		}
	}

	Ok(())
}
