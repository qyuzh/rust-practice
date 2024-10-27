/// P684, leetcode - Redundant Connection
/// keywords: union-find
pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    const N: usize = 1001;
    let mut union_find = UnionFind::new(N);
    for edge in edges {
        let x = edge[0];
        let y = edge[1];
        if union_find.find(x) == union_find.find(y) {
            return edge;
        }
        union_find.union(x, y);
    }
    unreachable!()
}

struct UnionFind {
    parent: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n as i32).collect();
        Self { parent }
    }

    fn find(&mut self, x: i32) -> i32 {
        if self.parent[x as usize] != x {
            self.parent[x as usize] = self.find(self.parent[x as usize]);
        }
        self.parent[x as usize]
    }

    fn union(&mut self, x: i32, y: i32) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_x as usize] = root_y;
        }
    }
}
