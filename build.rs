/******************************************************************************
* (c) 2005-2025 Copyright, Real-Time Innovations.  All rights reserved.       *
* No duplications, whole or partial, manual or electronic, may be made        *
* without express written permission.  Any such copies, or revisions thereof, *
* must display this notice unaltered.                                         *
* This code contains trade secrets of Real-Time Innovations, Inc.             *
******************************************************************************/

use std::{
    env,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

type Result<T> = std::result::Result<T, String>;
type Fallible = Result<()>;

fn main() -> Fallible {
    let out_dir = env::var("OUT_DIR")
        .map(PathBuf::from)
        .expect("OUT_DIR is set by Cargo");
    let lib_arch = compute_lib_arch()?;

    // Create appropriate library source based on environment variables
    let source: Box<dyn LibrarySource> =
        match LibraryProvisioner::select_from_environment()? {
            LibraryProvisioner::GitHub(github_source) => Box::new(github_source),
            LibraryProvisioner::Directory(dir_source) => Box::new(dir_source),
        };

    println!("Extracting connectorlibs from {}...", source.description());
    let link_path = source.extract_libraries(lib_arch, &out_dir)?;

    println!(r"cargo:rustc-link-search={}", link_path.display());
    println!(
        r"cargo:rerun-if-changed={}",
        concat!(env!("CARGO_MANIFEST_DIR"), "/docs/")
    );
    println!(
        r"cargo:rerun-if-changed={}",
        concat!(env!("CARGO_MANIFEST_DIR"), "/snippets/")
    );

    Ok(())
}

/// Determine the library architecture string based on the target OS and architecture.
///
/// We can't use `cfg!` macros here because Cargo build scripts may be cross-compiling.
pub fn compute_lib_arch() -> Result<&'static str> {
    let target_arch =
        env::var("CARGO_CFG_TARGET_ARCH").map_err(|_| "CARGO_CFG_TARGET_ARCH not set")?;
    let target_os =
        env::var("CARGO_CFG_TARGET_OS").map_err(|_| "CARGO_CFG_TARGET_OS not set")?;

    match target_os.as_str() {
        "windows" => match target_arch.as_str() {
            "x86_64" => Ok("win-x64"),
            arch => Err(format!("Unsupported Windows architecture: {}", arch)),
        },
        "linux" => match target_arch.as_str() {
            "x86_64" => Ok("linux-x64"),
            "aarch64" => Ok("linux-arm64"),
            arch => Err(format!("Unsupported Linux architecture: {}", arch)),
        },
        "macos" => match target_arch.as_str() {
            "x86_64" => Ok("osx-x64"), // Deprecated
            "aarch64" => Ok("osx-arm64"),
            arch => Err(format!("Unsupported macOS architecture: {}", arch)),
        },
        os => Err(format!("Unsupported operating system: {}", os)),
    }
}

/// Trait for different library source types.
///
/// Implementors provide methods to extract connector libraries from a given source,
/// as well as a human-readable description of how this is sourced.
trait LibrarySource {
    /// Extract the connector libraries for the specified architecture into the output directory.
    fn extract_libraries(&self, lib_arch: &str, output_dir: &Path) -> Result<PathBuf>;

    /// Provide a human-readable description of the source.
    fn description(&self) -> String;
}

/// Enum representing different ways to provision the connector libraries.
enum LibraryProvisioner {
    /// Fetch libraries from a GitHub release.
    GitHub(GitHubSource),

    /// Fetch libraries from a local directory.
    Directory(DirectorySource),
}

impl LibraryProvisioner {
    /// Select the appropriate library provisioner based on environment variables.
    pub fn select_from_environment() -> Result<Self> {
        const LIB_DIR_NAME: &str = "rticonnextdds-connector";
        const VERSION_ENV: &str = "RTI_CONNECTOR_VERSION";
        const DIR_ENV: &str = "RTI_CONNECTOR_DIR";
        const CARGO_ENV: &str = "CARGO_MANIFEST_DIR";
        const VERSION_FILE: &str =
            concat!(env!("CARGO_MANIFEST_DIR"), "/CONNECTOR_VERSION");

        println!("cargo:rerun-if-env-changed={}", VERSION_ENV);
        println!("cargo:rerun-if-env-changed={}", DIR_ENV);

        if let Ok(version) = env::var(VERSION_ENV) {
            /*
             * First choice:
             * Use version specified in RTI_CONNECTOR_VERSION to fetch from GitHub releases.
             */
            Ok(LibraryProvisioner::GitHub(GitHubSource::new(version)))
        } else if let Some(connector_lib_dir) = env::var(DIR_ENV)
            .ok()
            .map(PathBuf::from)
            .filter(|path| path.exists() && path.is_dir())
        {
            /*
             * Second choice:
             * Use local directory specified in RTI_CONNECTOR_DIR.
             */
            println!("cargo:rerun-if-changed={}", connector_lib_dir.display());
            Ok(LibraryProvisioner::Directory(DirectorySource::new(
                connector_lib_dir,
            )))
        } else if let Some(manifest_lib_dir) = env::var(CARGO_ENV)
            .ok()
            .map(|path_str| PathBuf::from(path_str).join(LIB_DIR_NAME))
            .filter(|path| path.exists() && path.is_dir())
        {
            /*
             * Third choice:
             * Use 'rticonnextdds-connector' directory in the Cargo manifest directory.
             */
            println!("cargo:rerun-if-changed={}", manifest_lib_dir.display());
            Ok(LibraryProvisioner::Directory(DirectorySource::new(
                manifest_lib_dir,
            )))
        } else if let Some(version) = std::fs::read_to_string(VERSION_FILE)
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
        {
            /*
             * Fallback:
             * Use version specified in VERSION file to fetch from GitHub releases.
             */
            println!("cargo:rerun-if-changed={}", VERSION_FILE);
            Ok(LibraryProvisioner::GitHub(GitHubSource::new(version)))
        } else {
            /*
             * Error scenario: VERSION file does not exist, cannot be read, or is empty after trimming.
             */
            Err(format!(
                "Environment variables {} and {} unset. {} doesn't contain native libraries. The file '{}' does not exist or is invalid.",
                VERSION_ENV, DIR_ENV, CARGO_ENV, VERSION_FILE
            ))
        }
    }
}

/// Source that fetches connector libraries from a GitHub release.
struct GitHubSource {
    version: String,
    api_root_uri: String,
    api_token: Option<String>,
}

impl LibrarySource for GitHubSource {
    fn extract_libraries(&self, lib_arch: &str, output_dir: &Path) -> Result<PathBuf> {
        let asset_url = self.fetch_release_asset_url()?;
        let zip_data = self.download_zip_data(&asset_url)?;
        self.extract_from_zip(zip_data, lib_arch, output_dir)
    }

    fn description(&self) -> String {
        format!("GitHub release version '{}'", self.version)
    }
}

impl GitHubSource {
    fn new(version: String) -> Self {
        println!("cargo:rerun-if-env-changed=GITHUB_TOKEN");

        let api_token = env::var("GITHUB_TOKEN")
            .ok()
            .map(|token| token.trim().to_string())
            .filter(|token| !token.is_empty());

        Self {
            version,
            api_root_uri:
                "https://api.github.com/repos/rticommunity/rticonnextdds-connector"
                    .to_string(),
            api_token,
        }
    }

    fn add_github_token_header<T>(
        &self,
        mut request: ureq::RequestBuilder<T>,
    ) -> ureq::RequestBuilder<T> {
        if let Some(token) = &self.api_token {
            request = request.header("Authorization", format!("Bearer {}", token));
        }

        request
    }

    fn fetch_release_asset_url(&self) -> Result<String> {
        let release_url = if self.version == "latest" {
            format!("{}/releases/latest", self.api_root_uri)
        } else {
            format!("{}/releases/tags/v{}", self.api_root_uri, self.version)
        };
        let release_json = self
            .add_github_token_header(ureq::get(&release_url))
            .header("Accept", "application/vnd.github+json")
            .call()
            .map_err(|e| {
                format!("Failed to fetch '{}' release info: {}", self.version, e)
            })?
            .into_body()
            .read_to_string()
            .map_err(|e| {
                format!("Failed to read '{}' release info: {}", self.version, e)
            })?;

        #[derive(Debug, serde::Deserialize)]
        struct Release {
            assets: Vec<Asset>,
        }

        #[derive(Debug, serde::Deserialize)]
        struct Asset {
            name: String,
            id: u64,
        }

        let release: Release = serde_json::from_str(&release_json)
            .map_err(|e| format!("Failed to parse release JSON: {}", e))?;

        let asset_id = release
            .assets
            .into_iter()
            .find(|a| a.name.starts_with("connectorlibs") && a.name.ends_with(".zip"))
            .map(|asset| asset.id)
            .ok_or_else(|| "No connectorlibs ZIP asset found in release".to_string())?;

        Ok(format!(
            "{}/releases/assets/{}",
            self.api_root_uri, asset_id
        ))
    }

    fn download_zip_data(&self, asset_url: &str) -> Result<Vec<u8>> {
        let request_body = self
            .add_github_token_header(ureq::get(asset_url))
            .header("Accept", "application/octet-stream")
            .call()
            .map_err(|e| format!("Failed to download asset: {}", e))?
            .into_body();

        let mut vec = Vec::new();
        request_body
            .into_reader()
            .read_to_end(&mut vec)
            .map_err(|e| format!("Failed to read asset data: {}", e))?;

        Ok(vec)
    }

    fn extract_from_zip(
        &self,
        zip_data: Vec<u8>,
        lib_arch: &str,
        output_dir: &Path,
    ) -> Result<PathBuf> {
        let lib_infix = format!("lib/{}/", lib_arch);
        let mut archive = zip::ZipArchive::new(Cursor::new(zip_data))
            .map_err(|e| format!("Failed to read ZIP: {}", e))?;

        let mut extracted_count = 0;
        let extraction_path = output_dir.join("lib").join(lib_arch);

        for i in 0..archive.len() {
            let file = archive
                .by_index(i)
                .map_err(|e| format!("Failed to access file #{} in ZIP: {}", i, e))?;

            let name = file.name();
            if !name.contains(&lib_infix) {
                continue;
            }

            // Extract the path after "rticonnextdds-connector/" and place directly in output_dir
            let relative_path =
                if let Some(stripped) = name.strip_prefix("rticonnextdds-connector/") {
                    stripped
                } else {
                    name // fallback to original name if prefix not found
                };
            let output_path = output_dir.join(relative_path);

            if file.is_dir() {
                std::fs::create_dir_all(&output_path).map_err(|e| {
                    format!(
                        "Failed to create directory '{}': {}",
                        output_path.display(),
                        e
                    )
                })?;
            } else {
                if let Some(parent) = output_path.parent() {
                    std::fs::create_dir_all(parent).map_err(|e| {
                        format!(
                            "Failed to create directory '{}': {}",
                            parent.display(),
                            e
                        )
                    })?;
                }

                let mut output_file =
                    std::fs::File::create(&output_path).map_err(|e| {
                        format!(
                            "Failed to create file '{}': {}",
                            output_path.display(),
                            e
                        )
                    })?;

                std::io::copy(&mut file.take(u64::MAX), &mut output_file).map_err(
                    |e| {
                        format!(
                            "Failed to write to file '{}': {}",
                            output_path.display(),
                            e
                        )
                    },
                )?;

                extracted_count += 1;
            }
        }

        if extracted_count == 0 {
            Err("No files were extracted from the ZIP".to_string())
        } else {
            Ok(extraction_path)
        }
    }
}

/// Source that fetches connector libraries from a local directory.
struct DirectorySource {
    source_path: PathBuf,
}

impl LibrarySource for DirectorySource {
    fn extract_libraries(&self, lib_arch: &str, output_dir: &Path) -> Result<PathBuf> {
        let src_dir = self.compute_source_path(lib_arch)?;

        std::fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create destination directory: {}", e))?;

        let copied_count =
            DirectorySource::copy_directory_recursive(&src_dir, output_dir)?;

        if copied_count == 0 {
            Err(format!(
                "No files or directories found in source directory '{}'",
                src_dir.display()
            ))
        } else {
            Ok(output_dir.to_path_buf())
        }
    }

    fn description(&self) -> String {
        format!("Local directory '{}'", self.source_path.display())
    }
}

impl DirectorySource {
    fn new(source_path: PathBuf) -> Self {
        Self { source_path }
    }

    fn compute_source_path(&self, lib_arch: &str) -> Result<PathBuf> {
        let lib_path = self.source_path.join("lib").join(lib_arch);

        if !lib_path.exists() {
            return Err(format!(
                "Source directory '{}' does not exist",
                lib_path.display()
            ));
        }

        if !lib_path.is_dir() {
            return Err(format!(
                "Source path '{}' is not a directory",
                lib_path.display()
            ));
        }

        Ok(lib_path)
    }

    fn copy_directory_recursive(src_dir: &Path, dest_dir: &Path) -> Result<usize> {
        std::fs::read_dir(src_dir)
            .map_err(|e| {
                format!(
                    "Failed to read source directory '{}': {}",
                    src_dir.display(),
                    e
                )
            })?
            .map(|entry| -> Result<usize> {
                let entry = entry
                    .map_err(|e| format!("Failed to read directory entry: {}", e))?;
                let src_path = entry.path();
                let dest_path = dest_dir.join(entry.file_name());

                if src_path.is_dir() {
                    std::fs::create_dir_all(&dest_path).map_err(|e| {
                        format!("Failed to create directory '{:?}': {}", dest_path, e)
                    })?;
                    let sub_items =
                        DirectorySource::copy_directory_recursive(&src_path, &dest_path)?;
                    Ok(1 + sub_items)
                } else if src_path.is_file() {
                    std::fs::copy(&src_path, &dest_path).map_err(|e| {
                        format!(
                            "Failed to copy file '{:?}' to '{:?}': {}",
                            src_path, dest_path, e
                        )
                    })?;
                    Ok(1)
                } else {
                    Ok(0) // Skip symlinks, etc.
                }
            })
            .try_fold(0, |acc, result| result.map(|count| acc + count))
    }
}
