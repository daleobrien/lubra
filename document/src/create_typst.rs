use itertools::Itertools;
use std::fs;
use std::io::BufRead;

pub fn create_typst_doc() {
    //let mut file = std::fs::File::create("./../typst/fragments.typ").unwrap();

    let mut paths: Vec<_> = std::fs::read_dir("./../typst/svgs/")
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    paths.sort_by_key(|dir| dir.file_name());

    let commands = paths
        .iter()
        .map(|path| {
            let file_name = path.file_name().into_string().unwrap();
            let name = file_name.split('.').next().unwrap();
            let name = name
                .split('-')
                .collect::<Vec<_>>()
                .join(" ")
                .chars()
                .filter(|c| c.is_digit(10) || c.is_whitespace())
                .collect::<String>();
            format!("  make(\"svgs/{}\", \"{}\")", file_name, name)
        })
        .join(",\n");

    let commands = format!("#import \"helper.typ\": *\n\n#grid(\n  columns: 4,\n{commands},\n)");
    fs::write("./../typst/fragments.typ", commands).expect("Unable to write file");
}
