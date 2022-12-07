use scan_fmt::*;
use std::collections::VecDeque;

fn main() {
    let input = include_str!("input.txt");
    part_one(input);
    part_two(input);
}

#[derive(PartialEq, Eq, Debug)]
enum Filesystem {
    File(String, u32),
    Dir(String, Vec<Box<Filesystem>>),
}

impl Filesystem {
    fn size(&self) -> u32 {
        match self {
            Filesystem::File(_, n) => *n,
            Filesystem::Dir(_, v) => v.iter().map(|x| x.size()).sum(),
        }
    }
}

fn simulate(commands: &str) -> Filesystem {
    fn recurse<'t>(wd: &mut Filesystem, commands: VecDeque<&'t str>) -> VecDeque<&'t str> {
        let mut commands = commands;
        let Some(command) = commands.pop_front() else {return commands;};
        if command == "$ cd /" {
            // we do nothing
        } else if command == "$ cd .." {
            // we're done here
            return commands;
        } else if let Ok(dirname) = scan_fmt!(command, "$ cd {}", String) {
            let Filesystem::Dir(_, children) = wd else {unreachable!("cd'd into file???")};
            for mut child in children.iter_mut() {
                if let Filesystem::Dir(n, _) = &**child {
                    if *n == dirname {
                        commands = recurse(&mut child, commands);
                        return recurse(wd, commands);
                    }
                }
            }
            let mut newdir = Filesystem::Dir(dirname, Vec::new());
            commands = recurse(&mut newdir, commands);
            children.push(Box::new(newdir));
        } else if command == "$ ls" {
            // we do nothing
        } else if let Ok((size, name)) = scan_fmt!(command, "{} {}", u32, String) {
            // TODO - check for existing file
            let Filesystem::Dir(_, children) = wd else {unreachable!("cd'd into file??")};
            let newfile = Filesystem::File(name, size);
            children.push(Box::new(newfile));
        } else if let Ok(dirname) = scan_fmt!(command, "dir {}", String) {
            // TODO - check for existing dir
            let Filesystem::Dir(_, children) = wd else {unreachable!("cd'd into file??")};
            let newfile = Filesystem::Dir(dirname, Vec::new());
            children.push(Box::new(newfile));
        } else {
            unreachable!("unknown command: '{}'", command);
        }
        // continue executing commands in the current dir
        recurse(wd, commands)
    }

    let mut fs = Filesystem::Dir(String::from("/"), Vec::new());
    let commands = commands.lines().collect::<VecDeque<_>>();
    recurse(&mut fs, commands);
    fs
}

fn search_big(fs: &Filesystem) -> u32 {
    match fs {
        Filesystem::File(_, _) => 0,
        d @ Filesystem::Dir(_, v) => {
            let mut self_size = d.size();
            if self_size > 100000 {
                self_size = 0;
            }
            v.iter().map(|x| search_big(&*x)).sum::<u32>() + self_size
        }
    }
}

fn part_one(input: &str) {
    let fs = simulate(input);
    let res = search_big(&fs);
    println!("{res}");
}

fn search_smallest_big(fs: &Filesystem) -> u32 {
    match fs {
        Filesystem::File(_, _) => u32::MAX,
        d @ Filesystem::Dir(_, v) => {
            let mut self_size = d.size();
            if self_size < 8381165 {
                self_size = u32::MAX;
            }
            v.iter()
                .map(|x| search_smallest_big(&*x))
                .min()
                .unwrap()
                .min(self_size)
        }
    }
}

fn part_two(input: &str) {
    let fs = simulate(input);
    let res = search_smallest_big(&fs);
    println!("{res}");
}
