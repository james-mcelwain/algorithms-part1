fn main() {
    let node_map = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut qf = QuickFind::new(node_map);
    qf = qf.union(1, 2);
    println!("{}{:?}", qf.connected(1, 2), qf.nodes);
}

#[derive(Debug)]
struct QuickFind {
    nodes: Vec<u8>
}

impl QuickFind {
    pub fn new(node_map: Vec<u8>) -> QuickFind {
        QuickFind {
            nodes: node_map
        }
    }
    pub fn connected(&self, x: usize, y: usize) -> bool {
        &self.nodes[x] == &self.nodes[y]
    }
    pub fn union(&self, x: usize, y: usize) -> QuickFind {
        let new_map = self.nodes.iter().map(|node| {
            if node == &self.nodes[x] {self.nodes[y]} else {*node}
        }).collect::<Vec<u8>>();
        QuickFind::new(new_map)
    }
}