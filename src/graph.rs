//graph mod holds the mod struct and the functions it operates on

type Vertex = usize;
type AdjacencyLists = Vec<Vec<Vertex>>;
type ListOfEdges = Vec<(Vertex,Vertex)>;
use std::collections::HashMap;

//It represents the graphs by holding the size of the graph and the adjacency list.
#[derive(Debug)]
pub struct Graph {
    pub n: usize,
    pub adjacency_list: AdjacencyLists,
} 

//reverses edges of the edge list
pub fn reverse_edges(list:&ListOfEdges) -> ListOfEdges {
    let mut new_list = Vec::new();
    for (u,v) in list {
        new_list.push((*v,*u));
    }
    return new_list
}

//graph impl statement
impl Graph{
    //adds directed edges to each adjacency list
    fn add_directed_edges(&mut self, edges:&ListOfEdges, index_map : HashMap<usize, usize>) {
        for (u,v) in edges {
            let idx = index_map.get(&u).expect("An error with finding index in graph.rs");
            self.adjacency_list[*idx].push(*v);
        }
    }

    //makes a new graph with directed edges
    fn create_directed(n:usize, edges:&ListOfEdges, index_map: HashMap<usize, usize>) -> Graph {
        let mut g = Graph{n,adjacency_list:vec![Vec::new();n]};
        g.add_directed_edges(edges, index_map);
        return g
    }

    //makes an undirected edge graph
    pub fn create_undirected(n:usize, edges:&ListOfEdges, index_map:HashMap<usize, usize>) -> Graph {
        let mut g = Self::create_directed(n, edges, index_map.clone());
        g.add_directed_edges(&reverse_edges(edges), index_map);
        return g
    }
}

