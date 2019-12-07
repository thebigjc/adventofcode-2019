use std::collections::HashMap;
use petgraph::{Graph,Direction};
use petgraph::graph::NodeIndex;
use petgraph::dot::{Dot, Config};
use std::fs;

fn main() {
  let s = fs::read_to_string("day6.txt").unwrap();
  let ret = one(&s);
  println!("{}", ret);
}

fn node_depth(g: &Graph<&str, &str>, ni: NodeIndex) -> u32 {
    let mut cnt = 0;
    for n in g.neighbors_directed(ni,Direction::Incoming) {
        cnt += node_depth(&g, n)+1;
    }
    cnt
}

fn one(s: &str) -> u32 {
    let mut m:HashMap<&str,NodeIndex> = HashMap::new();
    let mut g = Graph::<&str, &str>::new();
    for edges in s.split_whitespace() {
        let mut vec = Vec::new();

        for e in edges.split(")") {
            if !m.contains_key(e) {
                let n = g.add_node(e);
                m.insert(e, n);
            }
            vec.push(m[e]);
        }
        g.add_edge(vec[0], vec[1], s);
    }

    println!("{:?}", Dot::with_config(&g, &[Config::EdgeNoLabel]));

    let mut cnt = 0;

    for n in g.node_indices() {
        cnt += node_depth(&g, n);      
    }

    cnt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let s = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";
        let ret = one(s);

        assert_eq!(42, ret);
    }
}
