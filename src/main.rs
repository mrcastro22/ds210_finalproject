mod graph;
use crate::graph::*;

fn main() {
    let (retweet_vec, reply_vec, mention_vec) = read_file("src/data.txt");
    let before = SystemTime::now(); 
    println!("the average path length for retweeting is {:?}", average_path(retweet_vec.adjacency_list.len(), retweet_vec.adjacency_list));
    println!("the average path length for mentioning is {:?}", average_path(mention_vec.adjacency_list.len(), mention_vec.adjacency_list));
    println!("the average path lenth for replying is {:?}", average_path(reply_vec.adjacency_list.len(), reply_vec.adjacency_list));
    let after = SystemTime::now(); 
    let difference = after.duration_since(before); 
    let difference = difference.expect("Did the clock go back?"); 
    println!("Time it took: {:?}", difference);
}

use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::time::SystemTime;

type Vertex = usize;
type AdjacencyLists = Vec<Vec<Vertex>>;
type ListOfEdges = Vec<(Vertex,Vertex)>;

//an enum for the types of interactions we have
#[derive(Debug, Clone, Copy)]
enum InteractionType {
    Mention,
    Retweet,
    Reply,
}

//it makes 3 graphs from the txt file. Takes the txt file and outputs 3 graphs. No complex loops or algorithms
fn read_file(path: &str) -> (Graph, Graph, Graph) {
    let index_map = make_index_map(path);
    let mut edge_list_retweet : ListOfEdges = Vec::new();
    let mut edge_list_reply : ListOfEdges = Vec::new();
    let mut edge_list_mention : ListOfEdges = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let original_user = v[0].parse::<usize>().unwrap();
        let to_user = v[1].parse::<usize>().unwrap();
        let interaction_type = v[3].parse::<String>().unwrap();
        let mut interaction_enum = None;
        if interaction_type == String::from("MT") {
            interaction_enum = Some(InteractionType::Mention);
        }
        else if interaction_type == String::from("RT") {
            interaction_enum = Some(InteractionType::Retweet);
        }
        else if interaction_type == String::from("RE") {
            interaction_enum = Some(InteractionType::Reply);
        }
        match interaction_enum {
            Some(InteractionType::Mention) => {
                edge_list_mention.push((original_user, to_user));
            }
            Some(InteractionType::Reply) => {
                edge_list_reply.push((original_user, to_user));
            }
            Some(InteractionType::Retweet) => {
                edge_list_retweet.push((original_user, to_user));
            }
            None => println!("None type passed in")  
        }
    }

    let size = index_map.len();
    let retweet_vec : Graph = Graph::create_undirected(size, &edge_list_retweet, index_map.clone());
    let mention_vec : Graph = Graph::create_undirected(size, &edge_list_mention, index_map.clone());
    let reply_vec : Graph = Graph::create_undirected(size, &edge_list_reply, index_map.clone());

    return (retweet_vec, reply_vec, mention_vec)
}

//makes a hashmap that stores arbitrary indices for each userID. Returns a hashmap of two usizes.
fn make_index_map(path:&str) -> HashMap<usize, usize> {
    let mut counter = 0;
    let mut index_map : HashMap<usize, usize> = HashMap::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = std::io::BufReader::new(file).lines();
    for line in buf_reader {
        let line_str = line.expect("Error reading");
        let v: Vec<&str> = line_str.trim().split(' ').collect();
        let original_user = v[0].parse::<usize>().unwrap();
        if let None = index_map.get(&original_user) {
            index_map.insert(original_user, counter);
            counter += 1;
        }
        let to_user = v[1].parse::<usize>().unwrap();
        if let None = index_map.get(&to_user) {
            index_map.insert(to_user, counter);
            counter += 1;
        }
    }
    return index_map
}

//uses bfs, a vertex, and the adjacency list to calculate the longest distance from a certain point
fn compute_distance_bfs(start: Vertex, adjacency_list:&AdjacencyLists) -> usize {
    let index_map = make_index_map("src/data.txt");
    let mut counting_vector : Vec<usize> = Vec::new();
    let mut distance : Vec<Option<u32>> = vec![None;adjacency_list.len()+1];
    distance[start] = Some(0);
    let mut queue : VecDeque<Vertex> = VecDeque::new();
    queue.push_back(start);
    while let Some(v) = queue.pop_front() {
        for u in adjacency_list[v].iter() {
            //transforms the userID to the index its stored at in the adjacency list.
            let u_idx = index_map.get(&u).expect("There's an error finding the index");
            if let None = distance[*u_idx] {
                distance[*u_idx] = Some (distance[v].unwrap()+1);
                queue.push_back(*u_idx);
            }
        }
    }
    for v in 0..adjacency_list.len() {
        if let Some(_k) = distance[v] {
            counting_vector.push(distance[v].unwrap() as usize)
        }
    }
    counting_vector.sort_by(|a,b| b.cmp(&a));
    return counting_vector[0]
}

//finds the average length you can travel from one point for a graph.
fn average_path(n:usize,adjacency_list: Vec<Vec<usize>>) -> f64 {
    let mut counting_vector : Vec<usize> = Vec::new();
    for i in 0..n {
        if adjacency_list[i].len() != 0 {
            counting_vector.push(compute_distance_bfs(i, &adjacency_list));
        }
    }
    let mut counter = 0;
    for i in &counting_vector {
        counter += *i
    }
    return counter as f64/(counting_vector.len() as f64)
}

#[test]
fn does_reverse_edges_work() {
    let test_vec = vec!((9,0), (8,1), (7,2), (6,3));
    assert_eq!(reverse_edges(&test_vec), vec!((0,9), (1,8), (2,7), (3,6)));
}

#[test]
fn is_idx_map_7_long() {
    assert_eq!(make_index_map("src/test-set.txt").len(), 7);
}