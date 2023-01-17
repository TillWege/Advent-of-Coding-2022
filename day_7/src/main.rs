use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq)]
struct TreeNode {
    pub name: String,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
    pub size: u32,
    pub is_folder: bool,
}

impl TreeNode {
    pub fn new() -> TreeNode {
        return TreeNode {
            name: "".to_string(),
            children: vec![],
            parent: None,
            size: 0,
            is_folder: true,
        };
    }

    pub fn get_size(&self) -> u32{
        if !self.is_folder {
            return self.size;
        } else {
            let mut size = 0;
            for child in self.children.iter() {
                size += child.borrow().get_size();
            }
            return size;
        }
    }

    pub fn print(&self, level: usize) {
        println!("{}{}/{} ({})", " ".repeat(level * 2), self.get_parent_path(), self.name, self.get_size());
        if self.children.len() != 0  {
            for child in &self.children {
                child.borrow().print(level + 1);
            }
        }
    }

    pub fn get_parent_path(&self) -> String {
        if self.parent.is_none() {
            return "".to_string();
        } else {
            let parent = self.parent.as_ref().unwrap();
            let mut path = parent.borrow().get_parent_path();
            path.push_str(&parent.borrow().name);
            return path;
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("./inputs/day7.txt")?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)?;

    let fs = Rc::new(RefCell::new(TreeNode::new()));
    let mut current = Rc::clone(&fs);

    for line in contents.lines() {
        let mut parts = line.split(" ");
        let is_command = parts.next().unwrap();

        if is_command == "$" {
            let command = parts.next().unwrap();
            match command {
                "cd" => {
                    let path = parts.next().unwrap();
                    if path == ".." {
                        let current_clone = Rc::clone(&current);
                        current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                    } else {
                        let new_node = Rc::new(RefCell::new(TreeNode::new()));
                        current.borrow_mut().children.push(Rc::clone(&new_node));
                        {
                            let mut mut_child = new_node.borrow_mut();
                            mut_child.parent = Some(Rc::clone(&current));
                            mut_child.name = path.to_string();
                        }
                        current = new_node;
                    }
                }
                _ => {}
            }
        } else {
            if is_command != "dir" {
                let size: u32 = is_command.parse().unwrap();
                let name = parts.next().unwrap();
                let new_node = Rc::new(RefCell::new(TreeNode::new()));
                new_node.borrow_mut().is_folder = false;
                new_node.borrow_mut().size = size;
                new_node.borrow_mut().name = name.to_string();
                new_node.borrow_mut().parent = Some(Rc::clone(&current));
                current.borrow_mut().children.push(Rc::clone(&new_node));                
            }
        }
    }

    fs.borrow().print(0);

    current = Rc::clone(&fs);
    let mut task_1_size = 0;

    for child in current.borrow().children.iter() {
        task_1_size += get_task1_size(&child.borrow());
    }

    println!("Task 1: {}", task_1_size.to_string());

    let total_fs_space = 70000000;
    let needed_fs_space = 30000000; 

    let used_space = fs.borrow().get_size();
    let free_space = total_fs_space - used_space;

    let needed_space = needed_fs_space - free_space;
    dbg!(needed_space);
    println!("Task 2: {}", get_smallest_folder_with_size(&fs.borrow(), needed_space).unwrap());

    Ok(())
}

fn get_task1_size(node: &TreeNode) -> u32 {
    if !node.is_folder {
        return 0;
    } else {
        let mut total_size;
        if node.get_size() <= 100000 {
            total_size = node.get_size();
        } else {
            total_size = 0;
        }

        for child in node.children.iter() {
            total_size += get_task1_size(&child.borrow());
        }

        total_size
    }
}

fn get_smallest_folder_with_size(node: &TreeNode, min_size: u32) -> Option<u32>{
    let mut total_size = None;

    let node_size = node.get_size();
    if node_size > min_size {
        total_size = Some(node.get_size());
    }

    for child in node.children.iter() {
        if !child.borrow().is_folder {
            continue;
        }
        let child_size = get_smallest_folder_with_size(&child.borrow(), min_size);
        if let Some(size) = child_size {
            let current_size = total_size.unwrap_or(u32::MAX);
            if (size < current_size) && (size > min_size) {
                total_size = Some(size);
            }
        }
    }

    total_size
    
}