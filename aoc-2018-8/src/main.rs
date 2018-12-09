use std::collections::vec_deque::VecDeque;

struct Node<'a> {
    child_nodes: Vec<Node<'a>>,
    metadata: &'a [usize],
    size: usize
}

impl <'a> Node<'a> {
    fn sum(&self) -> usize {
        let sums: Vec<usize> = self.into_iter().map(|node: &Node| node.metadata.iter().sum()).collect();
        sums.into_iter().sum()
    }

    fn value(&self) -> usize {
        if self.child_nodes.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .into_iter()
                .map(|index| {
                    self.child_nodes
                        .get(*index - 1)
                        .map_or(0, |n| n.value())
                })
                .sum()
        }
    }
}

impl <'a> IntoIterator for &'a Node<'a> {
    type Item = &'a Node<'a>;
    type IntoIter = NodeIterator<'a>;

    fn into_iter(self) -> <Self as IntoIterator>::IntoIter {
        let mut queue = VecDeque::new();
        queue.push_back(&*self);
        NodeIterator { cur_nodes: queue }
    }
}

struct NodeIterator<'a> {
    cur_nodes: VecDeque<&'a Node<'a>>
}

impl <'a> Iterator for NodeIterator<'a> {
    type Item = &'a Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.cur_nodes.pop_front()?;
        self.cur_nodes.extend(&next.child_nodes);
        Some(next)
    }
}

fn get_node(nums: &[usize]) -> Node {
    let num_children = nums[0];
    let num_metadata = nums[1];

    let mut child_nodes = Vec::new();
    let mut next_child = 2;
    while child_nodes.len() < num_children as usize {
        let child = get_node(&nums[next_child..]);
        next_child += child.size;
        child_nodes.push(child);
    }
    let size = next_child + num_metadata as usize;
    let metadata = &nums[next_child .. size];
    Node { child_nodes, metadata, size }
}

fn part1(input: &str) -> usize {
    let nums: Vec<usize> = input.split(' ').map(str::parse).flatten().collect();

    let root_node = get_node(&nums.as_slice());
    root_node.sum()
}

fn part2(input: &str) -> usize {
    let nums: Vec<usize> = input.split(' ').map(str::parse).flatten().collect();

    let root_node = get_node(&nums.as_slice());
    root_node.value()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part2(input));
}

#[test]
fn test_part1() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    assert_eq!(part1(input), 138);
}

#[test]
fn test_part2() {
    let input = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";
    assert_eq!(part2(input), 66);
}
