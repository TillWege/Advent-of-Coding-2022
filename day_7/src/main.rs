const default_path: &str = "/";

enum Element{
    Folder,
    File
}

struct File {
    name: String,
    size: usize
}

struct Folder {
    name: String,
    elements: Vec<Element>,
}

impl Folder {
    fn add_element(& mut self, elem: Element) {
        self.elements.push(elem)
    }

    fn calc_size(& self) -> usize {
        for elem in &self.elements {
            

        };

        0
    }
}

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./inputs/day7.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;

    let mut file_system = Folder {
        name: default_path.to_string(),
        elements: Vec::new(),
    };

    let mut current_path: Vec<String> = vec![default_path.to_string()];


    for line in contents.lines() {
        if line.starts_with('&') {
            let mut parts = line.split_whitespace();
            parts.next();

            if parts.next().unwrap() == "cd" {
                let cd_arg = parts.next().unwrap();
                if cd_arg == default_path {
                    current_path.drain(1..);
                } else if cd_arg == ".." {
                    current_path.pop();
                } else {
                    current_path.push(cd_arg.to_string());
                }
            }
        } else {
            // ls output
            let mut ls_parts = line.split_whitespace();
            let possible_size = ls_parts.next().unwrap();
            if let Ok(size) = possible_size.parse::<usize>() {

            } else {

            }




        }
    }

    Ok(())
}
