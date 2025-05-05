type Vertex = usize;
type AdjacencyLists = Vec<Vec<Vertex>>;
type ListOfEdges = Vec<(Vertex,Vertex)>;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub adjacency_list: AdjacencyLists,
} 

fn reverse_edges(list:&ListOfEdges) -> ListOfEdges {
    let mut new_list = Vec::new();
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    return new_list
}

impl Graph{
    fn add_directed_edges(&mut self, edges:&ListOfEdges, index_map : HashMap<usize, usize>) {
        for (u,v) in edges {
            let idx = index_map.get(&u).expect("An error with finding index in graph.rs");
            self.adjacency_list[*idx].push(*v);
        }
    }

    fn create_directed(n:usize, edges:&ListOfEdges, index_map: HashMap<usize, usize>) -> Graph {
        let mut g = Graph{n,adjacency_list:vec![Vec::new();n]};
        g.add_directed_edges(edges, index_map);
        return g
    }

    pub fn create_undirected(n:usize, edges:&ListOfEdges, index_map:HashMap<usize, usize>) -> Graph {
        let mut g = Self::create_directed(n, edges, index_map.clone());
        g.add_directed_edges(&reverse_edges(edges), index_map);
        return g
    }
}