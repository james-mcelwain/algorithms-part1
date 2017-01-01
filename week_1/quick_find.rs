fn main() {
    let node_map = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut qf = QuickFind::new(node_map);
    qf = qf.union(1, 2);
    println!("{}{:?}", qf.connected(1, 2), qf);
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
    pub fn connected(&self, x: u8, y: u8) -> bool {
        &self.nodes[x as usize] == &self.nodes[y as usize]
    }
    pub fn union(&self, x: u8, y: u8) -> QuickFind {
        let new_map = self.nodes.iter().map(|node| {
            if node == &self.nodes[x as usize] {self.nodes[y as usize]} else {*node}
        }).collect::<Vec<u8>>();
        QuickFind::new(new_map)
    }
}