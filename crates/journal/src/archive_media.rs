//! Download already-discovered media URLs into a journal entry's
//! Entry-Attached Evidence `assets/` directory, at whatever resolution the
//! URL itself serves. Port of the journal CLI's
//! `archive-media/{action,derive-filename,resolve-entry-paths}.ts`.
//!
//! The one deliberate network access in an otherwise fully offline crate:
//! there is no way to save "the image at this URL" without fetching it. This
//! module never discovers URLs on its own (typically a human finds them via a
//! browser) and never renders a page -- only downloads bytes from URLs it is
//! handed.

use std::path::PathBuf;

use common::{Error, Result};
use url::Url;

use crate::date::Date;

/// Where an existing entry (found by its dated slug) lives, and where its
/// assets directory is. Mirrors `entry.rs`'s `YYYY/MM-Month/DD-Weekday`
/// convention from the slug string alone, since archiving media targets an
/// entry that already exists -- there is no `EntrySpec` to ask.
pub fn resolve_entry_paths(slug: &str) -> Result<(PathBuf, PathBuf)> {
    let bytes = slug.as_bytes();
    let valid = slug.len() >= 11
        && bytes[0..4].iter().all(u8::is_ascii_digit)
        && bytes[4] == b'-'
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[7] == b'-'
        && bytes[8..10].iter().all(u8::is_ascii_digit)
        && bytes[10] == b'-';
    if !valid {
        return Err(Error::Config(format!(
            "\"{slug}\" is not a dated entry slug (expected YYYY-MM-DD-<rest>)"
        )));
    }
    // Digits were just validated above, so these parses cannot fail.
    let year: i32 = slug[0..4].parse().expect("validated digits");
    let month: u32 = slug[5..7].parse().expect("validated digits");
    let day: u32 = slug[8..10].parse().expect("validated digits");
    let date = Date::new(year, month, day);

    let dir = PathBuf::from(format!("{year:04}"))
        .join(format!("{month:02}-{}", date.month_name()))
        .join(format!("{day:02}-{}", date.weekday_name()));
    let entry_path = dir.join(format!("{slug}.md"));
    let assets_dir = dir.join(slug).join("assets");
    Ok((entry_path, assets_dir))
}

const EXTENSION_BY_CONTENT_TYPE: &[(&str, &str)] = &[
    ("image/jpeg", "jpg"),
    ("image/png", "png"),
    ("image/webp", "webp"),
    ("image/gif", "gif"),
    ("image/svg+xml", "svg"),
    ("application/pdf", "pdf"),
];

fn slugify(s: &str) -> String {
    let mut out = String::new();
    let mut last_dash = false;
    for c in s.chars().flat_map(char::to_lowercase) {
        if c.is_ascii_alphanumeric() {
            out.push(c);
            last_dash = false;
        } else if !last_dash && !out.is_empty() {
            out.push('-');
            last_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        "asset".to_string()
    } else {
        out
    }
}

/// The extension a CDN-served media URL actually carries, checked in the
/// order that mattered in practice: a `?format=` query param (e.g.
/// pbs.twimg.com media URLs, which have none in the path at all), then a
/// plain path suffix.
fn extension_from_url(url: &Url) -> Option<String> {
    if let Some(format) = url.query_pairs().find(|(k, _)| k == "format") {
        return Some(format.1.to_lowercase());
    }
    let last_segment = url.path_segments()?.next_back()?;
    let (_, ext) = last_segment.rsplit_once('.')?;
    if (2..=4).contains(&ext.len()) && ext.chars().all(|c| c.is_ascii_alphanumeric()) {
        Some(ext.to_lowercase())
    } else {
        None
    }
}

fn basename_from_url(url: &Url) -> String {
    let last_segment = url
        .path_segments()
        .and_then(|mut s| s.next_back())
        .unwrap_or("");
    match last_segment.rsplit_once('.') {
        Some((stem, ext))
            if (2..=4).contains(&ext.len()) && ext.chars().all(|c| c.is_ascii_alphanumeric()) =>
        {
            stem
        }
        _ => last_segment,
    }
    .to_string()
}

/// Build a sane, collision-resistant asset filename: `<prefix>-<NN>-<name>.<ext>`.
/// The extension prefers the URL's own suffix or CDN format param, falling
/// back to the response's content-type; errors when neither yields one
/// rather than guessing wrong.
pub fn derive_filename(
    index: u32,
    prefix: &str,
    url: &str,
    name: Option<&str>,
    content_type: Option<&str>,
) -> Result<String> {
    let parsed =
        Url::parse(url).map_err(|e| Error::Config(format!("invalid URL \"{url}\": {e}")))?;
    let padded = format!("{index:02}");
    let name = slugify(name.unwrap_or(&basename_from_url(&parsed)));
    let ext = extension_from_url(&parsed).or_else(|| {
        content_type.and_then(|ct| {
            EXTENSION_BY_CONTENT_TYPE
                .iter()
                .find(|(k, _)| *k == ct)
                .map(|(_, v)| v.to_string())
        })
    });
    let Some(ext) = ext else {
        return Err(Error::Config(format!(
            "cannot determine a file extension for {url} (content-type: {})",
            content_type.unwrap_or("unknown")
        )));
    };
    Ok(format!("{prefix}-{padded}-{name}.{ext}"))
}

/// Bytes fetched from a URL, plus the response's own content-type header
/// (used as a fallback for extension detection).
pub struct FetchedBytes {
    pub bytes: Vec<u8>,
    pub content_type: Option<String>,
}

/// The default fetcher: a blocking GET via `ureq`. A separate function (not
/// inlined into `archive_media`) so tests can substitute a stub instead of
/// hitting the network.
pub fn fetch_bytes(url: &str) -> Result<FetchedBytes> {
    let response = ureq::get(url)
        .call()
        .map_err(|e| Error::Config(format!("GET {url} failed: {e}")))?;
    let content_type = response
        .header("content-type")
        .map(|h| h.split(';').next().unwrap_or(h).trim().to_lowercase());
    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|e| Error::Config(format!("reading response body from {url}: {e}")))?;
    Ok(FetchedBytes {
        bytes,
        content_type,
    })
}

/// One request to archive a set of already-discovered media URLs against a
/// single existing entry.
pub struct ArchiveMediaArgs<'a> {
    /// The entry's dated slug, e.g. `2026-09-22-jev-ai-memory-longmemeval-benchmark`.
    pub slug: &'a str,
    /// Media URLs to download, already discovered (e.g. via a browser).
    pub urls: &'a [String],
    /// Optional filename stems, positionally aligned with `urls`. Must match
    /// its length if given.
    pub names: Option<&'a [String]>,
    /// Filename prefix. Defaults to `"media"`.
    pub prefix: &'a str,
    /// Overwrite a destination file that already exists.
    pub force: bool,
}

pub struct SavedAsset {
    pub url: String,
    pub path: PathBuf,
    pub bytes: usize,
}

/// Download `args.urls` into the entry's assets directory, resolved from
/// `args.slug`. `root` is the journal root the entry and its assets live
/// under (almost always `.`); `fetch` is injected so tests never touch the
/// network.
pub fn archive_media(
    root: &std::path::Path,
    args: &ArchiveMediaArgs,
    fetch: impl Fn(&str) -> Result<FetchedBytes>,
    mut log: impl FnMut(&str),
) -> Result<Vec<SavedAsset>> {
    if args.urls.is_empty() {
        return Err(Error::Config(
            "no URLs given -- nothing to archive".to_string(),
        ));
    }
    if let Some(names) = args.names {
        if names.len() != args.urls.len() {
            return Err(Error::Config(format!(
                "--name given {} time(s) but {} URL(s) were given -- pass one --name per URL, or none",
                names.len(),
                args.urls.len()
            )));
        }
    }

    let (entry_path, assets_dir) = resolve_entry_paths(args.slug)?;
    let entry_abs = root.join(&entry_path);
    if !entry_abs.exists() {
        return Err(Error::Config(format!(
            "entry not found: {} -- check the slug",
            entry_path.display()
        )));
    }

    let mut saved = Vec::new();
    for (i, url) in args.urls.iter().enumerate() {
        let name = args.names.and_then(|n| n.get(i)).map(String::as_str);
        log(&format!("fetching {url}..."));
        let fetched = fetch(url)?;
        let filename = derive_filename(
            (i + 1) as u32,
            args.prefix,
            url,
            name,
            fetched.content_type.as_deref(),
        )?;
        let rel_path = assets_dir.join(&filename);
        let abs_path = root.join(&rel_path);

        if !args.force && abs_path.exists() {
            return Err(Error::Config(format!(
                "{} already exists -- pass --force to overwrite, or remove it first",
                rel_path.display()
            )));
        }

        if let Some(parent) = abs_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| Error::io(parent, e))?;
        }
        std::fs::write(&abs_path, &fetched.bytes).map_err(|e| Error::io(&abs_path, e))?;
        log(&format!(
            "saved {} ({} bytes)",
            rel_path.display(),
            fetched.bytes.len()
        ));
        saved.push(SavedAsset {
            url: url.clone(),
            path: rel_path,
            bytes: fetched.bytes.len(),
        });
    }

    Ok(saved)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolve_entry_paths_splits_year_month_day_weekday_from_slug() {
        // 2026-09-22 is a Tuesday.
        let (entry, assets) = resolve_entry_paths("2026-09-22-jev-ai-memory").unwrap();
        assert_eq!(
            entry,
            PathBuf::from("2026/09-September/22-Tuesday/2026-09-22-jev-ai-memory.md")
        );
        assert_eq!(
            assets,
            PathBuf::from("2026/09-September/22-Tuesday/2026-09-22-jev-ai-memory/assets")
        );
    }

    #[test]
    fn resolve_entry_paths_rejects_a_non_dated_slug() {
        assert!(resolve_entry_paths("not-a-dated-slug").is_err());
        assert!(resolve_entry_paths("2026-09-slug").is_err());
    }

    #[test]
    fn derive_filename_prefers_format_query_param() {
        let name = derive_filename(
            1,
            "media",
            "https://pbs.twimg.com/media/ABC123?format=jpg&name=large",
            None,
            None,
        )
        .unwrap();
        assert_eq!(name, "media-01-abc123.jpg");
    }

    #[test]
    fn derive_filename_falls_back_to_path_suffix() {
        let name =
            derive_filename(2, "media", "https://example.com/foo/bar.png", None, None).unwrap();
        assert_eq!(name, "media-02-bar.png");
    }

    #[test]
    fn derive_filename_falls_back_to_content_type() {
        let name = derive_filename(
            3,
            "shot",
            "https://example.com/no-extension-here",
            Some("Screenshot One"),
            Some("image/png"),
        )
        .unwrap();
        assert_eq!(name, "shot-03-screenshot-one.png");
    }

    #[test]
    fn derive_filename_errors_when_no_extension_can_be_determined() {
        let err = derive_filename(1, "media", "https://example.com/mystery", None, None);
        assert!(err.is_err());
    }

    #[test]
    fn archive_media_requires_at_least_one_url() {
        let tmp = tempfile::tempdir().unwrap();
        let args = ArchiveMediaArgs {
            slug: "2026-09-22-slug",
            urls: &[],
            names: None,
            prefix: "media",
            force: false,
        };
        let err = archive_media(tmp.path(), &args, |_| unreachable!(), |_| {});
        assert!(err.is_err());
    }

    #[test]
    fn archive_media_rejects_mismatched_name_count() {
        let tmp = tempfile::tempdir().unwrap();
        let args = ArchiveMediaArgs {
            slug: "2026-09-22-slug",
            urls: &["https://example.com/a.png".to_string()],
            names: Some(&["one".to_string(), "two".to_string()]),
            prefix: "media",
            force: false,
        };
        let err = archive_media(tmp.path(), &args, |_| unreachable!(), |_| {});
        assert!(err.is_err());
    }

    #[test]
    fn archive_media_rejects_a_missing_entry() {
        let tmp = tempfile::tempdir().unwrap();
        let args = ArchiveMediaArgs {
            slug: "2026-09-22-slug",
            urls: &["https://example.com/a.png".to_string()],
            names: None,
            prefix: "media",
            force: false,
        };
        let err = archive_media(tmp.path(), &args, |_| unreachable!(), |_| {});
        assert!(err.is_err());
    }

    #[test]
    fn archive_media_downloads_and_writes_into_assets_dir() {
        let tmp = tempfile::tempdir().unwrap();
        let entry_dir = tmp.path().join("2026/09-September/22-Tuesday");
        std::fs::create_dir_all(&entry_dir).unwrap();
        std::fs::write(entry_dir.join("2026-09-22-slug.md"), "# Slug\n").unwrap();

        let args = ArchiveMediaArgs {
            slug: "2026-09-22-slug",
            urls: &["https://example.com/photo.jpg".to_string()],
            names: None,
            prefix: "media",
            force: false,
        };
        let saved = archive_media(
            tmp.path(),
            &args,
            |_url| {
                Ok(FetchedBytes {
                    bytes: vec![1, 2, 3, 4],
                    content_type: None,
                })
            },
            |_| {},
        )
        .unwrap();

        assert_eq!(saved.len(), 1);
        assert_eq!(saved[0].bytes, 4);
        let written = std::fs::read(tmp.path().join(&saved[0].path)).unwrap();
        assert_eq!(written, vec![1, 2, 3, 4]);
    }

    #[test]
    fn archive_media_refuses_to_overwrite_without_force() {
        let tmp = tempfile::tempdir().unwrap();
        let entry_dir = tmp.path().join("2026/09-September/22-Tuesday");
        std::fs::create_dir_all(&entry_dir).unwrap();
        std::fs::write(entry_dir.join("2026-09-22-slug.md"), "# Slug\n").unwrap();
        let assets_dir = tmp
            .path()
            .join("2026/09-September/22-Tuesday/2026-09-22-slug/assets");
        std::fs::create_dir_all(&assets_dir).unwrap();
        std::fs::write(assets_dir.join("media-01-photo.jpg"), b"existing").unwrap();

        let args = ArchiveMediaArgs {
            slug: "2026-09-22-slug",
            urls: &["https://example.com/photo.jpg".to_string()],
            names: None,
            prefix: "media",
            force: false,
        };
        let err = archive_media(
            tmp.path(),
            &args,
            |_url| {
                Ok(FetchedBytes {
                    bytes: vec![9],
                    content_type: None,
                })
            },
            |_| {},
        );
        assert!(err.is_err());
    }

    #[test]
    fn archive_media_overwrites_when_forced() {
        let tmp = tempfile::tempdir().unwrap();
        let entry_dir = tmp.path().join("2026/09-September/22-Tuesday");
        std::fs::create_dir_all(&entry_dir).unwrap();
        std::fs::write(entry_dir.join("2026-09-22-slug.md"), "# Slug\n").unwrap();
        let assets_dir = tmp
            .path()
            .join("2026/09-September/22-Tuesday/2026-09-22-slug/assets");
        std::fs::create_dir_all(&assets_dir).unwrap();
        std::fs::write(assets_dir.join("media-01-photo.jpg"), b"existing").unwrap();

        let args = ArchiveMediaArgs {
            slug: "2026-09-22-slug",
            urls: &["https://example.com/photo.jpg".to_string()],
            names: None,
            prefix: "media",
            force: true,
        };
        let saved = archive_media(
            tmp.path(),
            &args,
            |_url| {
                Ok(FetchedBytes {
                    bytes: vec![9, 9],
                    content_type: None,
                })
            },
            |_| {},
        )
        .unwrap();
        let written = std::fs::read(tmp.path().join(&saved[0].path)).unwrap();
        assert_eq!(written, vec![9, 9]);
    }
}
