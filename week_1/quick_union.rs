fn main () {
    let mut quf = QuickUnionUF::new(10);
    println!("{:?}", quf);
    quf.union(1u8, 2u8);
    println!("{:?}", quf)
}

#[derive(Debug)]
struct QuickUnionUF {
    nodes: Vec<u8>
}

impl QuickUnionUF {
    pub fn new (size: u8) -> QuickUnionUF {
        let mut nodes = Vec::new();
        for x in 0..size {
            nodes.push(x);
        }

        QuickUnionUF {
            nodes: nodes
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