use aidoku::{
	prelude::format, std::{defaults::defaults_get, ObjectRef, String, Vec}, Chapter, MangaStatus
};

extern crate alloc;
use alloc::collections::BTreeMap;

pub fn get_lang_code() -> Option<String> {
	if let Ok(lang) = defaults_get("languages") {
		if let Ok(languages) = lang.as_array() {
			if let Ok(language) = languages.get(0).as_string() {
				return Some(language.read());
			}
		}
	}
	None
}

pub fn data_from_json(data: &ObjectRef, key: &str) -> String {
	match data.get(key).as_string() {
		Ok(str) => str.read(),
		Err(_) => String::new(),
	}
}

#[allow(clippy::too_many_arguments)]
pub fn get_search_url(
	api_url: String,
	query: String,
	included_tags: Vec<String>,
	excluded_tags: Vec<String>,
	demographic_tags: Vec<String>,
	manga_type: String,
	sort_by: String,
	completed: String,
	page: i32,
) -> String {
	let mut url = format!("{}/v1.0/search?page={}&tachiyomi=true", api_url, page);
	if !query.is_empty() {
		url.push_str(&format!("&t=true&q={}", query.replace(' ', "%20")))
	}
	if !demographic_tags.is_empty() {
		for tag in demographic_tags {
			url.push_str(&format!("&demographic={}", tag));
		}
	}

	if !included_tags.is_empty() || !excluded_tags.is_empty() {
		if excluded_tags.is_empty() {
			for tag in included_tags {
				url.push_str(&format!("&genres={}", tag));
			}
		} else if !included_tags.is_empty() && !excluded_tags.is_empty() {
			for tag in included_tags {
				url.push_str(&format!("&genres={}", tag));
			}
			for tag in excluded_tags {
				url.push_str(&format!("&excludes={}", tag));
			}
		} else {
			for tag in excluded_tags {
				url.push_str(&format!("&excludes={}", tag));
			}
		}
	}
	if !sort_by.is_empty() {
		url.push_str(&format!("&sort={}", sort_by));
	}
	if !manga_type.is_empty() {
		url.push_str(&format!("&country={}", manga_type));
	}
	if !completed.is_empty() {
		url.push_str(&format!("&completed={}", completed));
	}
	url
}

pub fn get_listing_url(api_url: String, list_name: String, page: i32) -> String {
	let lang_code = get_lang_code().unwrap_or_else(|| String::from("en"));
	let url = match list_name.as_str() {
		"Hot" => format!(
			"{}/chapter?lang={}&page={}&order=hot&tachiyomi=true",
			api_url, lang_code, page
		),
		_ => format!(
			"{}/chapter?lang={}&page={}&order=new&tachiyomi=true",
			api_url, lang_code, page
		),
	};
	url
}

pub fn manga_status(status: i64) -> MangaStatus {
	match status {
		1 => MangaStatus::Ongoing,
		2 => MangaStatus::Completed,
		_ => MangaStatus::Unknown,
	}
}

pub fn group_by<T, K, F>(items: Vec<T>, mut key_extractor: F) -> BTreeMap<K, Vec<T>>
where
    T: Clone,
    K: Ord + Clone,
    F: FnMut(&T) -> K,
{
    let mut groups: BTreeMap<K, Vec<T>> = BTreeMap::new();

    for item in items {
        let key = key_extractor(&item);
        groups.entry(key).or_insert_with(Vec::new).push(item.clone());
    }

    groups
}

pub fn take_chapter(group: Vec<Chapter>) -> Option<Chapter> {
    if let Ok(scanlator_priorities) = defaults_get("scanlatorPriorities") {
        if let Ok(scanlator_priorities) = scanlator_priorities.as_string() {
            let priorities = scanlator_priorities.read();
            let priorities: Vec<&str> = priorities.split(',').map(|s| s.trim()).collect();
            for priority in priorities {
                if let Some(chapter) = group.iter().find(|c| c.scanlator == priority) {
                    return Some(chapter.clone());
                }
            }
        }
    }

    group.first().cloned()
}
