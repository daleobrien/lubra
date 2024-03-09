use itertools::Itertools;
use std::fs;
use std::io::BufRead;

pub fn create_typst_doc(path_name: &str, typst_filename: &str, columns: usize) {
    //let mut file = std::fs::File::create("./../typst/fragments.typ").unwrap();

    let mut paths: Vec<_> = std::fs::read_dir(format!("./../typst/{path_name}/"))
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
            format!("  make(\"{path_name}/{file_name}\", \"{name}\")")
        })
        .join(",\n");

    let commands =
        format!("#import \"helper.typ\": *\n\n#grid(\n  columns: {columns},\n{commands},\n)");
    fs::write(format!("./../typst/{typst_filename}"), commands).expect("Unable to write file");
}
