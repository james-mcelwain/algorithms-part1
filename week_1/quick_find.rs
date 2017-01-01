fn main() {
    let mut qf = QuickFind::new(10);
    qf.union(1, 2);
    println!("{}{:?}", qf.connected(1, 2), qf);
}

#[derive(Debug)]
struct QuickFind {
    nodes: Vec<u8>
}

impl QuickFind {
    pub fn new(size: u8) -> QuickFind {
        let mut nodes = Vec::new();
        for x in 0..size {
            nodes.push(x);
        }

        QuickFind {
            nodes: nodes
        }
    }
    pub fn connected(&self, x: u8, y: u8) -> bool {
        &self.nodes[x as usize] == &self.nodes[y as usize]
    }
    pub fn union(&mut self, x: u8, y: u8) {
        self.nodes[x as usize] = self.nodes[y as usize];
    }
}