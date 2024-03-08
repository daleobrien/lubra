use itertools::Itertools;

pub fn create_typst_doc() {
    let mut file = std::fs::File::create("./../typst/fragments.typ").unwrap();
    // list each file in /typst/svgs
    let paths: Vec<_> = std::fs::read_dir("./../typst/svgs/")
        .unwrap()
        .map(|r| r.unwrap())
        .collect();
    // sort paths
    let paths = paths.sorted_by_key(|dir| dir.file_name());

    for path in paths {
        // Get the base file name
        let file_name = path.unwrap().file_name();
        println!(">>> {:?}", file_name);
    }
}
