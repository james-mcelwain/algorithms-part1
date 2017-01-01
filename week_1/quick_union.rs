fn main () {
    let mut quf = QuickUnionUF::new(vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    println!("{:?}", quf);
    quf.union(1u8, 2u8);
    println!("{:?}", quf)
}

#[derive(Debug)]
struct QuickUnionUF {
    nodes: Vec<u8>
}

impl QuickUnionUF {
    pub fn new (node_map: Vec<u8>) -> QuickUnionUF {
        QuickUnionUF {
            nodes: node_map
        }
    }
    pub fn root (&self, i: u8) -> u8 {
        let mut _i = i;
        while _i != self.nodes[_i as usize] {_i = self.nodes[_i as usize]}
        _i
    }
    pub fn connected (&self, x: usize, y: usize) -> bool {
        &self.nodes[x as usize] == &self.nodes[y as usize]
    }
    pub fn union (&mut self, x: u8, y: u8) {
        let x_val = self.root(x);
        let y_val = self.root(y);
        self.nodes[x_val as usize] = y_val;
    }
}