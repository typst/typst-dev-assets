//! Assets for the testing & docs generation of the Typst compiler.
//!
//! These are not part of the main compiler crate to keep its size down.

macro_rules! files {
    ($($path:literal),* $(,)?) => {
        const FILES: &[(&str, &[u8])] = &[
            $((
                $path,
                include_bytes!(concat!("../files/", $path)).as_slice(),
            )),*
        ];
    }
}

/// Get a file by path.
pub fn get(path: &str) -> Option<&'static [u8]> {
    let slot;
    let mut path = path;
    if path.contains('\\') {
        slot = path.replace('\\', "/");
        path = &slot;
    }

    FILES.iter().find(|&&(p, _)| p == path).map(|&(_, d)| d)
}

/// Get a file by name, without specifying the directory.
pub fn get_by_name(filename: &str) -> Option<&'static [u8]> {
    FILES
        .iter()
        .find(|&&(p, _)| p.split('/').next_back() == Some(filename))
        .map(|&(_, d)| d)
}

/// Get all font data.
///
/// This does not include the fonts that are already part of `typst-assets`.
pub fn fonts() -> impl Iterator<Item = &'static [u8]> {
    FILES
        .iter()
        .filter(|&(p, _)| p.starts_with("fonts"))
        .map(|&(_, d)| d)
}

files! {
    "bib/bad.bib",
    "bib/scifi-authors.yaml",
    "bib/works.bib",
    "bib/works_too.bib",
    "data/bad.csv",
    "data/bad.json",
    "data/bad.toml",
    "data/bad.xml",
    "data/bad.yaml",
    "data/big-number.json",
    "data/details.toml",
    "data/example.csv",
    "data/example.xml",
    "data/hello.xml",
    "data/monday.json",
    "data/moore.csv",
    "data/toml-types.toml",
    "data/tuesday.json",
    "data/yaml-types.yaml",
    "data/zoo.csv",
    "data/zoo.json",
    "fonts/Asana-Math.otf",
    "fonts/CascadiaCode-Regular.ttf",
    "fonts/Concrete-Math.otf",
    "fonts/Garamond-Math.otf",
    "fonts/IBMPlexMath-Regular.otf",
    "fonts/IBMPlexSans-Bold.ttf",
    "fonts/IBMPlexSans-Light.ttf",
    "fonts/IBMPlexSans-Medium.ttf",
    "fonts/IBMPlexSans-Regular.ttf",
    "fonts/IBMPlexSansCondensed-Regular.ttf",
    "fonts/IBMPlexSansDevanagari-Regular.ttf",
    "fonts/IBMPlexSerif-Regular.ttf",
    "fonts/InriaSerif-Bold.ttf",
    "fonts/InriaSerif-BoldItalic.ttf",
    "fonts/InriaSerif-Italic.ttf",
    "fonts/InriaSerif-Regular.ttf",
    "fonts/LibertinusMath-Regular.otf",
    "fonts/NotoColorEmoji-Regular-COLR.subset.ttf",
    "fonts/NotoSansArabic-Regular.ttf",
    "fonts/NotoSansMath-Regular.ttf",
    "fonts/NotoSansSymbols2-Regular.ttf",
    "fonts/NotoSansThai-Regular.ttf",
    "fonts/NotoSerifCJKjp-Regular.otf",
    "fonts/NotoSerifCJKkr-Regular.otf",
    "fonts/NotoSerifCJKsc-Bold.otf",
    "fonts/NotoSerifCJKsc-Regular.otf",
    "fonts/NotoSerifCJKtc-Bold.otf",
    "fonts/NotoSerifCJKtc-Regular.otf",
    "fonts/NotoSerifHebrew-Bold.ttf",
    "fonts/NotoSerifHebrew-Regular.ttf",
    "fonts/PTSans-Regular.ttf",
    "fonts/PennstanderMath-Regular.otf",
    "fonts/Roboto-Regular.ttf",
    "fonts/STIXTwoMath-Regular.otf",
    "fonts/SourceSerif4-Regular.otf",
    "fonts/TwitterColorEmoji.ttf",
    "fonts/Ubuntu-Regular.ttf",
    "fonts/XITSMath-Regular.otf",
    "fonts/texgyrebonum-math.otf",
    "images/bad.svg",
    "images/base14-fonts.pdf",
    "images/chart-bad-deuteranopia.png",
    "images/chart-bad-regular.png",
    "images/chart-good.png",
    "images/chinese.svg",
    "images/color-contrast.png",
    "images/cylinder.svg",
    "images/diagram.svg",
    "images/diagrams.pdf",
    "images/docs.svg",
    "images/f2t.jpg",
    "images/f2t.png",
    "images/glacier.jpg",
    "images/graph.png",
    "images/heron.jpg",
    "images/linked.svg",
    "images/logo.svg",
    "images/matplotlib.pdf",
    "images/molecular.jpg",
    "images/monkey.svg",
    "images/pattern.svg",
    "images/relative.svg",
    "images/rhino.png",
    "images/small.gif",
    "images/small.jpeg",
    "images/small.jpg",
    "images/small.png",
    "images/small.webp",
    "images/star.pdf",
    "images/tetrahedron.svg",
    "images/tiger.jpg",
    "images/typing.jpg",
    "plugins/hello-mut.wasm",
    "plugins/hello.wasm",
    "plugins/plugin-oob.wasm",
    "screenshots/1-writing-app.png",
    "screenshots/1-writing-upload.png",
    "screenshots/2-formatting-autocomplete.png",
    "screenshots/3-advanced-paper.png",
    "screenshots/3-advanced-team-settings.png",
    "syntaxes/SExpressions.sublime-syntax",
    "text/bad.txt",
    "text/example.html",
    "text/hello.txt",
    "themes/halcyon.tmTheme",
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use super::*;

    #[test]
    fn test_unix_like() {
        assert!(get("data/zoo.csv").is_some());
        assert!(get("data/zoos.csv").is_none());
    }

    #[test]
    fn test_windows_like() {
        assert!(get("data\\zoo.csv").is_some());
        assert!(get("data\\zoos.csv").is_none());
    }

    #[test]
    fn test_list_sorted() {
        for window in FILES.windows(2) {
            let (a, _) = window[0];
            let (b, _) = window[1];
            if a > b {
                panic!("{a:?} and {b:?} are out of order");
            }
        }
    }

    #[test]
    fn test_all_files_included() {
        let root = Path::new("files");
        walk(root, &mut |path| {
            let stringified = path
                .strip_prefix(root)
                .unwrap()
                .to_string_lossy()
                .replace(std::path::MAIN_SEPARATOR_STR, "/");
            let data = std::fs::read(&path).unwrap();
            if get(&stringified) != Some(data.as_slice()) {
                panic!("{} is not listed in {}", path.display(), file!());
            }
        })
    }

    fn walk(dir: &Path, f: &mut impl FnMut(PathBuf)) {
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(&path, f);
            } else if let Some(stem) = path.file_stem()
                && let Some(stem_str) = stem.to_str()
                && !stem_str.starts_with(".")
            {
                f(path);
            }
        }
    }
}
