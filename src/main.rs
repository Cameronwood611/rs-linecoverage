use std::cmp;


#[derive(Copy, Clone)]
struct Line {
    x: i32,
    y: i32
}

#[derive(Debug)]
pub enum NodeValue {
    Str(&'static str),
    Int(i32)
}

#[derive(Debug)]
pub struct Node {
    value: NodeValue,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>
}

impl Node {
    pub fn new(value: NodeValue, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Self {
        Node { value: value, left: left, right: right }
    }
}

fn add_line(line: Line, mut node: Node) -> Node {
    let x = line.x;
    let y = line.y;

    if y <= x { // already covered space
        return node;
    }

    match node.value {
        NodeValue::Str(s) => {
            match s {
                "empty" =>  Node::new( //wow
                    NodeValue::Int(x),
                    Some(Box::new(Node::new(NodeValue::Str("empty"), None, None))),
                    Some(Box::new(Node::new(
                        NodeValue::Int(y),
                        Some(Box::new(Node::new(NodeValue::Str("covered"), None, None))),
                        Some(Box::new(Node::new(NodeValue::Str("empty"), None, None)))
                    )))
                ),
                "covered" => node,
                _ => node // should never happen but rust complains about not covering every case
            }
               
                

        },
        
        NodeValue::Int(n) => {
            node.left = Some(Box::new(add_line(Line { x: cmp::min(x, n), y: cmp::min(y, n) }, *node.left.unwrap())));
            node.right = Some(Box::new(add_line(Line { x: cmp::max(x, n), y: cmp::max(y, n) }, *node.right.unwrap())));
            return node
        }
    }
}

fn line_coverage(lines: Vec<Line>) {

    let mut root = Node::new(NodeValue::Str("empty"), None, None);
    for line in lines {
        root = add_line(line, root);
    }
}

fn main() {
    let l1 = Line { x: 2, y: 5 };
    let l2 = Line { x: 7, y: 9 };
    let l3 = Line { x: 6, y: 15 };

    let lines = vec![l1, l2, l3];
    line_coverage(lines);
}
