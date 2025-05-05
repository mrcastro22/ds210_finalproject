mod graph;
use crate::graph::Graph;

fn main() {
    let (retweet_vec, reply_vec, mention_vec) = read_file("src/testing.txt");
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

#[derive(Debug, Clone, Copy)]
enum InteractionType {
    Mention,
    Retweet
    Reply,
}

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