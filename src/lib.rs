mod memes;
mod options;
mod registry;
mod tags;

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, fs::OpenOptions, io::Write, path::PathBuf, sync::Once};

    use super::registry::MemeDeclaration;

    static INIT: Once = Once::new();

    fn init_meme_home() {
        INIT.call_once(|| unsafe {
            std::env::set_var("MEME_HOME", env!("CARGO_MANIFEST_DIR"));
        });
    }

    fn log_path() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/preview-test.log")
    }

    fn log_line(line: &str) {
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_path())
        {
            let _ = writeln!(file, "{line}");
            let _ = file.flush();
        }
        println!("{line}");
    }

    fn is_png_or_gif(bytes: &[u8]) -> bool {
        bytes.starts_with(&[0x89, b'P', b'N', b'G']) || bytes.starts_with(b"GIF")
    }

    #[test]
    fn preview_all_memes() {
        init_meme_home();
        let _ = std::fs::remove_file(log_path());

        let mut failures = Vec::new();
        let mut passed = 0usize;
        let mut total = 0usize;

        for declaration in inventory::iter::<MemeDeclaration> {
            total += 1;
            let meme = (declaration.builder)();
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                meme_generator_core::meme::Meme::generate_preview(&*meme, HashMap::new())
            }));
            match result {
                Ok(Ok(bytes)) if bytes.len() >= 32 && is_png_or_gif(&bytes) => {
                    passed += 1;
                    log_line(&format!("ok  {} ({} bytes)", declaration.name, bytes.len()));
                }
                Ok(Ok(bytes)) => {
                    let msg = format!(
                        "{}: invalid output ({} bytes)",
                        declaration.name,
                        bytes.len()
                    );
                    log_line(&format!("FAIL {msg}"));
                    failures.push(msg);
                }
                Ok(Err(err)) => {
                    let msg = format!("{}: {err}", declaration.name);
                    log_line(&format!("FAIL {msg}"));
                    failures.push(msg);
                }
                Err(_) => {
                    let msg = format!("{}: panic", declaration.name);
                    log_line(&format!("FAIL {msg}"));
                    failures.push(msg);
                }
            }
        }

        log_line(&format!("preview {passed}/{total} passed"));
        assert!(total > 0, "no memes registered");
        assert!(
            failures.is_empty(),
            "preview failed ({}/{}):\n{}",
            failures.len(),
            total,
            failures.join("\n")
        );
    }
}
