use std::{cell::RefCell, collections::HashMap, path::Path, rc::Rc};

#[derive(Debug)]
enum NodeType {
    Dir,
    File,
}

impl std::fmt::Display for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    name: String,
    size: usize,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(
        node_type: NodeType,
        name: &str,
        size: usize,
        parent: Option<Rc<RefCell<Node>>>,
    ) -> Self {
        Node {
            node_type,
            name: String::from(name),
            size,
            parent,
            children: vec![],
        }
    }

    fn append(&mut self, new_node: Rc<RefCell<Node>>) {
        self.children.push(new_node);
    }

    fn cd(&mut self, dir_name: &str) -> Rc<RefCell<Node>> {
        if dir_name == ".." {
            let parent = self.parent.as_mut().unwrap();
            return Rc::clone(parent);
        }
        let dir = self
            .children
            .iter()
            .find(|node| node.borrow_mut().name == dir_name)
            .unwrap();
        Rc::clone(dir)
    }

    fn calc_size(&self) -> usize {
        let mut tree_size = 0;

        for node in self.children.iter() {
            let child = node.borrow_mut();
            match child.node_type {
                NodeType::Dir => {
                    tree_size += child.calc_size();
                }
                NodeType::File => {
                    tree_size += child.size;
                }
            }
        }

        tree_size
    }

    fn path(&self) -> String {
        match &self.parent {
            Some(parent) => {
                let n = &*parent.borrow();
                let np = n.path();
                let p = Path::new(&np);
                String::from(p.join(&self.name).to_str().unwrap())
            }
            None => self.name.clone(),
        }
    }

    fn dir_sizes(&self) -> HashMap<String, usize> {
        let mut map: HashMap<String, usize> = HashMap::new();
        map.insert(self.path(), self.calc_size());

        for node in self.children.iter() {
            let child = node.borrow();
            match child.node_type {
                NodeType::Dir => map.extend(child.dir_sizes()),
                NodeType::File => {}
            }
        }

        map
    }

    fn ls(&self) -> String {
        self.ls_with_indent(0)
    }

    fn ls_with_indent(&self, mut indent: usize) -> String {
        let mut s: String = self.to_string();
        indent += 1;
        for node in self.children.iter() {
            s += "\n";
            s += &" ".repeat(indent * 4).to_string();
            let child = node.borrow_mut();
            match child.node_type {
                NodeType::Dir => {
                    s = s + &child.ls_with_indent(indent);
                }
                NodeType::File => {
                    s = s + &child.to_string();
                }
            }
        }
        s
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self.node_type {
            NodeType::Dir => format!("- {} (dir)", self.name),
            NodeType::File => format!("- {} (file, size={})", self.name, self.size),
        };
        write!(f, "{}", s)
    }
}

fn main() {
    let input = include_str!("../../../inputs/day-7/input.txt");
    let root = parse_tree(input);

    println!("{}", root.borrow().ls());
    println!("{}", root.borrow().calc_size());

    let sizes = root.borrow().dir_sizes();
    let part1: usize = sizes
        .iter()
        .filter(|(_dir, size)| **size <= 100000)
        .fold(0, |acc, (_dir, size)| acc + size);
    println!("{}", part1);
}

fn parse_tree(input: &str) -> Rc<RefCell<Node>> {
    let root = Rc::new(RefCell::new(Node::new(NodeType::Dir, "/", 0, None)));
    let mut cwd = Rc::clone(&root);

    let mut liter = input.lines();
    // skip first cd /
    liter.next();

    for line in liter {
        let parts: Vec<&str> = line.split(' ').collect();
        if parts[0].starts_with('$') {
            let cmd = parts[1];
            if cmd == "cd" {
                let dir = parts[2];
                let d = cwd.borrow_mut().cd(dir);
                cwd = d;
            }
        } else if line.starts_with("dir") {
            let new_dir = parts[1];
            let mut new_node = Node::new(NodeType::Dir, new_dir, 0, Some(Rc::clone(&cwd)));
            new_node.parent = Some(Rc::clone(&cwd));
            cwd.borrow_mut().append(Rc::new(RefCell::new(new_node)));
        } else {
            let size = parts[0];
            let file_name = parts[1];
            let new_node = Node::new(
                NodeType::File,
                file_name,
                size.parse::<usize>().unwrap(),
                Some(Rc::clone(&cwd)),
            );
            cwd.borrow_mut().append(Rc::new(RefCell::new(new_node)));
        }
    }

    root
}
