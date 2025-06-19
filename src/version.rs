// version.rs

use reqwest;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct AurRpcResponse {
    results: Vec<AurPackageInfo>,
}

#[derive(Deserialize, Debug)]
struct AurPackageInfo {
    // Checking the AUR RPC v5 documentation, the field for version is usually named "Version" or
    // "version".
    // If the field is named differently in the actual JSON response, you may need to adjust this.
    // Usually, for AUR packages, the version field is named "Version".
    #[serde(rename = "Version")]
    pub version: String,
}

fn fetch_aur_version() -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    let pkg_name = "neocash"; // Убедитесь, что это точное имя вашего пакета на AUR
    let url = format!("https://aur.archlinux.org/rpc/v5/info/{}", pkg_name);

    let response = client
        .get(&url)
        .send()
        .map_err(|e| format!("Failed to send request to AUR API: {}", e))?
        .error_for_status()
        .map_err(|e| format!("AUR API returned an error: {}", e))?
        .json::<AurRpcResponse>()
        .map_err(|e| format!("Failed to parse AUR API response: {}", e))?;

    // Check if we have at least one result
    if let Some(package_info) = response.results.into_iter().next() {
        Ok(package_info.version)
    } else {
        Err(format!("Package '{}' not found in AUR", pkg_name))
    }
}

// Function to extract the base version from a version string
pub fn get_base_version_str(ver_str: &str) -> String {
    ver_str.splitn(2, '-').next().unwrap_or(ver_str).to_string()
}

pub fn check_for_updates() -> Result<String, String> {
    let current_version = env!("CARGO_PKG_VERSION");

    let base_current_version = get_base_version_str(current_version);

    let cache_dir = dirs::cache_dir().ok_or("Не удалось найти директорию кеша")?;

    let cache_file = cache_dir.join("neocash/version_cache");

    if let Ok(cached) = std::fs::read_to_string(&cache_file) {
        if let Some((timestamp, full_cached_version)) = cached.split_once('|') {
            if let Ok(ts) = timestamp.parse::<u64>() {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .map_err(|e| e.to_string())?
                    .as_secs();

                // Check if the cached version is still valid
                let base_cached_version = get_base_version_str(full_cached_version);
                if now - ts < 3600 && base_cached_version == base_current_version {
                    return Ok(full_cached_version.to_string()); // Return the full version from cache
                }
            }
        }
    }

    // Получение свежей версии с AUR
    let aur_ver = fetch_aur_version()?;

    // Update the cache file with the full version from AUR
    std::fs::create_dir_all(cache_file.parent().unwrap())
        .map_err(|e| format!("Failed to create cache dir: {}", e))?;

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| e.to_string())?
        .as_secs();

    std::fs::write(&cache_file, format!("{}|{}", now, aur_ver))
        .map_err(|e| format!("Failed to write cache: {}", e))?;

    Ok(aur_ver) // Return the full version from AUR
}
