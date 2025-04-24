use crate::lock;
use eks_validator::structs::{Metadata, ServerIndex};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub(crate) fn generate_server_index(
	project_root: &PathBuf,
	all_metadata: &Vec<Metadata>,
	lock: &HashMap<String, HashMap<String, String>>,
) {
	let mut index_list = vec![];

	for metadata in all_metadata {
		let id = match lock::get_lock_entry_id(lock, &metadata.extension.slug) {
			Some(id) => id.to_string(),
			None => {
				log::warn!(
                    "Skipping {} — no ID found in metadata-lock",
                    metadata.extension.slug
                );
				continue;
			}
		};

		let extension =  &metadata.extension;

		let index = ServerIndex {
			id: id.clone(),
			name: extension.name.clone(),
			icon: extension.icon.clone(),
			language: extension.language.clone(),
			version_code: extension.version_code,
			version_name: extension.version_name.clone(),
		};

		index_list.push(index);
	}

	let json = match serde_json::to_string_pretty(&index_list) {
		Ok(json) => json,
		Err(e) => {
			log::error!("Failed to serialize global index: {}", e);
			return;
		}
	};


	let output_path = project_root.join("index.json");

	if let Err(e) = fs::write(&output_path, json) {
		log::error!("Failed to write global index.json: {}", e);
	} else {
		log::info!("🌐 Global index written to {}", output_path.display());
	}
}
