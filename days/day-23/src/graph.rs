use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

use lib::debugln;

pub struct Graph {
    pub nodes: HashSet<Rc<String>>,
    pub links: HashMap<Rc<String>, HashSet<Rc<String>>>,
}

impl Graph {
    pub fn from(input: &str) -> Self {
        let mut graph = Graph {
            nodes: HashSet::new(),
            links: HashMap::new(),
        };

        for line in input.lines() {
            let (a, b) = line.split_once("-").unwrap();

            let a_idx = Rc::new(a.to_string());
            let b_idx = Rc::new(b.to_string());

            graph.insert(&a_idx);
            graph.insert(&b_idx);

            graph.link(&a_idx, &b_idx);
            graph.link(&b_idx, &a_idx);
        }

        graph
    }

    pub fn insert(&mut self, index: &Rc<String>) {
        if !self.nodes.contains(index) {
            self.nodes.insert(index.clone());
        }
    }

    pub fn link(&mut self, a: &Rc<String>, b: &Rc<String>) {
        let node_links = self.links.entry(a.clone()).or_default();

        if !node_links.contains(b) {
            node_links.insert(b.clone());
        }
    }

    pub fn find_largest_connected_set(&self) -> (usize, HashSet<Rc<String>>) {
        let cliques = self.bron_kerbosch();

        let max_clique = cliques
            .into_iter()
            .max_by_key(|clique| clique.len())
            .unwrap();

        (max_clique.len(), max_clique)
    }

    pub fn bron_kerbosch(&self) -> Vec<HashSet<Rc<String>>> {
        self._bron_kerbosch(&HashSet::new(), &self.nodes, &HashSet::new())
    }

    fn _bron_kerbosch(
        &self,
        r: &HashSet<Rc<String>>,
        p: &HashSet<Rc<String>>,
        x: &HashSet<Rc<String>>,
    ) -> Vec<HashSet<Rc<String>>> {
        debugln!("BK: r={:?}, p={:?}, x={:?}", r, p, x);

        if p.is_empty() && x.is_empty() {
            debugln!("<- {:?}", r);
            return vec![r.clone()];
        }

        if p.is_empty() {
            return vec![];
        }

        let pivot_u = p
            .iter()
            .max_by_key(|&node| self.links.get(node).map(|l| l.len()).unwrap_or(0))
            .unwrap();

        debugln!("\tu = {}", pivot_u);

        let n_u = self.links.get(pivot_u).unwrap();

        debugln!("\tn_u = {:?}", n_u);

        let mut results = vec![];
        let mut p = p.clone();
        let mut x = x.clone();

        for v in p
            .iter()
            .filter(|&n| !n_u.contains(n))
            .cloned()
            .collect::<Vec<Rc<String>>>()
        {
            let n_v = self.links.get(&v).unwrap();

            debugln!("v = {}", v);
            debugln!("n_v = {:?}", n_v);

            let mut new_r = r.clone();
            new_r.insert(v.clone());

            let new_p = p.iter().filter(|&v| n_v.contains(v)).cloned().collect();
            let new_x = x.iter().filter(|&v| n_v.contains(v)).cloned().collect();

            debugln!("-> r={:?}, p={:?}, x={:?}", new_r, new_p, new_x);

            results.append(&mut self._bron_kerbosch(&new_r, &new_p, &new_x));

            p.remove(&v);
            x.insert(v.clone());
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bron_kerbosch() {
        let graph = Graph::from(
            r#"
1-5
1-2
2-5
2-3
5-4
4-3
6-4
        "#
            .trim(),
        );

        let r = graph.bron_kerbosch();

        let mut cliques: Vec<Vec<String>> = r
            .iter()
            .map(|s| s.iter().map(|s| s.to_string()).collect())
            .collect();

        cliques.iter_mut().for_each(|set| set.sort());

        assert!(cliques.contains(&vec!["2".to_string(), "3".to_string()]));
        assert!(cliques.contains(&vec!["3".to_string(), "4".to_string()]));
        assert!(cliques.contains(&vec!["4".to_string(), "5".to_string()]));
        assert!(cliques.contains(&vec!["4".to_string(), "6".to_string()]));
        assert!(cliques.contains(&vec!["1".to_string(), "2".to_string(), "5".to_string()]));
    }
}
