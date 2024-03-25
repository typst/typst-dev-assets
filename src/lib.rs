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
        .find(|&&(p, _)| p.split('/').last() == Some(filename))
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
    "bib/works_too.bib",
    "bib/works.bib",
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
    "fonts/FiraMath-Regular.otf",
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
    "fonts/NotoColorEmoji.ttf",
    "fonts/NotoSansArabic-Regular.ttf",
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
    "fonts/Roboto-Regular.ttf",
    "fonts/TwitterColorEmoji.ttf",
    "fonts/Ubuntu-Regular.ttf",
    "images/bad.svg",
    "images/chinese.svg",
    "images/cylinder.svg",
    "images/diagram.svg",
    "images/docs.svg",
    "images/f2t.jpg",
    "images/glacier.jpg",
    "images/graph.png",
    "images/logo.svg",
    "images/molecular.jpg",
    "images/monkey.svg",
    "images/pattern.svg",
    "images/rhino.png",
    "images/tetrahedron.svg",
    "images/tiger.jpg",
    "images/typing.jpg",
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
}
