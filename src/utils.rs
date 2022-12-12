use std::fs;

pub trait Solve {
    fn process_input(&mut self, path: &str);
    fn part1(&mut self);
    fn part2(&mut self);
}

pub fn read_file(fp: &str) -> String {
    let mut path = std::env::current_dir().unwrap();
    path = path.join(format!("src/{}", fp));
    fs::read_to_string(path).unwrap()
}

#[allow(dead_code)]
pub fn bootstrap() {
    for day in 1..26 {
        let dir_path = format!("src/day{:0>2}", day);
        if fs::create_dir(&dir_path).is_ok() {
            let src_file = format!("{}/mod.rs", &dir_path);
            fs::File::create(format!("{}/input.txt", &dir_path)).unwrap();
            fs::File::create(&src_file).unwrap();

            let template = fs::read_to_string("src/template.rs").unwrap();
            let output = template.replace("dayX", &format!("day{:0>2}", day));
            fs::write(&src_file, output).unwrap();
        }
    }
}