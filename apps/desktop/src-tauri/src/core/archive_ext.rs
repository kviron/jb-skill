//! Подмножество известных расширений архивов модов (ориентир — Vortex `src/renderer/src/util/archives.ts`).
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModArchiveFormat {
    Zip,
    SevenZ,
    Rar,
}

/// Формат по расширению файла (только то, что ядро умеет распаковывать).
pub fn mod_archive_format(path: &Path) -> Option<ModArchiveFormat> {
    let ext = path.extension()?.to_str()?.to_lowercase();
    Some(match ext.as_str() {
        "zip" => ModArchiveFormat::Zip,
        "7z" => ModArchiveFormat::SevenZ,
        "rar" => ModArchiveFormat::Rar,
        _ => return None,
    })
}

/// Подходит ли файл под установку как архив мода (MVP: zip / 7z / rar).
pub fn path_looks_like_mod_archive(path: &Path) -> bool {
    mod_archive_format(path).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn recognizes_zip_7z_rar() {
        assert!(path_looks_like_mod_archive(Path::new("mod.zip")));
        assert!(path_looks_like_mod_archive(Path::new("C:/x/a.7z")));
        assert!(path_looks_like_mod_archive(Path::new("b.RAR")));
        assert!(!path_looks_like_mod_archive(Path::new("readme.txt")));
    }
}
