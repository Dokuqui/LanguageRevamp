use std::cmp::Ordering;

pub fn compare_versions(installed: &str, latest: &str) -> Ordering {
    let clean_installed = installed.trim_start_matches("go");
    let clean_latest = latest.trim_start_matches("go");

    let installed_parts: Vec<u32> = clean_installed
        .split('.')
        .filter_map(|p| p.parse().ok())
        .collect();
    let latest_parts: Vec<u32> = clean_latest
        .split('.')
        .filter_map(|p| p.parse().ok())
        .collect();

    installed_parts.cmp(&latest_parts)
}
