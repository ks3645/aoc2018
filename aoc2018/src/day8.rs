use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use utils;
use utils::Part;

pub fn solve(part: Part) -> i32 {
    let mut input = String::new();
    utils::read_input_to_string(&mut input, 8).unwrap();

    do_the_thing(input, part)
}

fn do_the_thing(input: String, part: Part) -> i32 {
    let data: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    let data_map = read_data_iteratively(&data);

    match part {
        Part::One => data_map
            .values()
            .map(|metadata| metadata.iter().map(|d| d.value()).sum::<usize>())
            .sum::<usize>() as i32,
        Part::Two => {
            let root  = data_map.keys().filter_map(|k| if k.id == 0 {Some(k)} else {None}).next().unwrap();

            resolve_pointer_value_iteratively(root, &data_map) as i32
        }
    }
}

fn resolve_pointer_value_iteratively(node: &NodeHeader, data: &HashMap<NodeHeader, Vec<MetadataType>>) -> usize {
    let mut stack: Vec<NodeHeader> = vec![];

    stack.push(node.clone());

    let mut value: usize = 0;
    'node: loop {
        let active_node = match stack.last() {
            Some(h) => h.clone(),
            None => break 'node,
        };

        println!("Active Node: {:?}", active_node);

        if active_node.num_children == 0 {
            let val = data
                .get(&active_node)
                .unwrap()
                .iter()
                .map(|d| d.value())
                .sum::<usize>();
            println!("No Children, adding: {:?}", val);
            value += val;
            stack.pop();
        } else {

            stack.pop();
            let mut children: Vec<NodeHeader> = data
                .keys()
                .filter_map(|n| { if let Some(id) = n.parent { if id == active_node.id { Some(n.clone()) } else { None } } else { None }})
                .collect();
            children.sort();

            for active_ptr in data.get(&active_node).unwrap().iter() {
                println!("Checking child: {:?}", active_ptr);
                if active_ptr.value() <= children.len()  {
                    stack.push(children[active_ptr.value() - 1].clone());
                }
            }
        }
    }

    value
}

#[derive(Debug, Copy, Clone)]
struct NodeHeader {
    id: i32,
    num_children: usize,
    num_metadata: usize,
    parent: Option<i32>,
}

impl Hash for NodeHeader {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Ord for NodeHeader {
    fn cmp(&self, other: &NodeHeader) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for NodeHeader {
    fn partial_cmp(&self, other: &NodeHeader) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl NodeHeader {
    fn new(id: i32) -> NodeHeader {
        NodeHeader {
            id,
            num_children: 0,
            num_metadata: 0,
            parent: None,
        }
    }
}

impl Eq for NodeHeader {}

impl PartialEq for NodeHeader {
    fn eq(&self, other: &NodeHeader) -> bool {
        self.id == other.id
    }
}

#[derive(Debug)]
enum MetadataType {
    Value(usize),
    Pointer(usize),
}

impl MetadataType {
    fn value(&self) -> usize {
        match self {
            MetadataType::Value(val) => val.clone(),
            MetadataType::Pointer(ptr) => ptr.clone(),
        }
    }
}

fn read_data_iteratively(data: &Vec<usize>) -> HashMap<NodeHeader, Vec<MetadataType>> {
    let mut stack: Vec<NodeHeader> = vec![];
    let mut data_ptr = 0;

    let mut child_counts: HashMap<NodeHeader, usize> = HashMap::new();

    let mut result_map = HashMap::new();

    let mut next_id = 0;

    let first_header = NodeHeader {
        id: next_id,
        num_children: data[data_ptr],
        num_metadata: data[data_ptr + 1],
        parent: None,
    };
    stack.push(first_header);
    child_counts.insert(first_header, first_header.num_children);

    next_id += 1;
    data_ptr += 2;

    'read: loop {
        let header = match stack.last() {
            Some(h) => h.clone(),
            None => break 'read,
        };

        {
            let child_count = child_counts.get(&header).unwrap();
            if child_count == &0 {
                let metadata: Vec<MetadataType> = data[data_ptr..data_ptr + header.num_metadata]
                    .iter()
                    .map(|data| {
                        if header.num_children == 0 {
                            MetadataType::Value(data.clone())
                        } else {
                            MetadataType::Pointer(data.clone())
                        }
                    }).collect();
                data_ptr += header.num_metadata;
                result_map.entry(header).or_insert(metadata);
                stack.pop();
                continue 'read;
            }
        }

        {
            let new_header = NodeHeader {
                id: next_id,
                num_children: data[data_ptr],
                num_metadata: data[data_ptr + 1],
                parent: Some(header.id),
            };
            next_id += 1;
            data_ptr += 2;

            stack.push(new_header);

            child_counts.insert(new_header, new_header.num_children);
            let count_to_decrement = child_counts.get_mut(&header).unwrap();
            *count_to_decrement -= 1;
        }
    }

    result_map
}
