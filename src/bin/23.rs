use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(23);

type Node = String;
type Graph = HashMap<Node, HashSet<Node>>;

fn bron_kerbosch1(
    graph: &Graph,
    r: &mut HashSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    cliques: &mut Vec<HashSet<String>>,
) {
    if p.is_empty() && x.is_empty() {
        cliques.push(r.clone());
    } else {
        let p_copy = p.clone();
        for v in p_copy.iter() {
            let empty_set = HashSet::new();
            let neighbors = graph.get(v).unwrap_or(&empty_set);
            let mut new_r = r.clone();
            new_r.insert(v.to_string());

            let mut new_p = p.intersection(&neighbors).cloned().collect();
            let mut new_x = x.intersection(&neighbors).cloned().collect();

            bron_kerbosch1(graph, &mut new_r, &mut new_p, &mut new_x, cliques);
            p.remove(v);
            x.insert(v.to_string());
        }
    }
}

fn get_clique_name(clique: &HashSet<Node>) -> String {
    clique.iter().sorted().join(",")
}

fn find_cliques_of_size_3(graph: &Graph) -> HashMap<String, HashSet<Node>> {
    let mut cliques = HashMap::new();

    for (vertex, neighbors) in graph.iter() {
        for neigh in neighbors.iter() {
            let neighbors1 = graph.get(neigh).unwrap();
            for neigh1 in neighbors1.iter() {
                let neighbors2 = graph.get(neigh1).unwrap();
                if neighbors2.contains(vertex) {
                    let mut clique = HashSet::new();
                    clique.insert(vertex.to_string());
                    clique.insert(neigh.to_string());
                    clique.insert(neigh1.to_string());

                    let clique_name = get_clique_name(&clique);
                    cliques.insert(clique_name, clique);
                }
            }
        }
    }
    cliques
}

fn get_password(nodes: &HashSet<Node>) -> String {
    nodes.iter().sorted().join(",")
}

fn create_graph(input: &str) -> Graph {
    let mut graph: Graph = HashMap::new();
    for line in input.lines() {
        let (n1, n2) = line.split_once("-").unwrap();
        graph
            .entry(n1.to_string())
            .or_insert_with(|| HashSet::new())
            .insert(n2.to_string());

        graph
            .entry(n2.to_string())
            .or_insert_with(|| HashSet::new())
            .insert(n1.to_string());
    }
    graph
}

pub fn part_one(input: &str) -> Option<u32> {
    let g = create_graph(input);
    let mut cliques3 = find_cliques_of_size_3(&g);
    cliques3.retain(|_, nodes| nodes.iter().any(|node| node.starts_with("t")));
    Some(cliques3.len() as u32)
}

pub fn part_two(input: &str) -> Option<String> {
    let g = create_graph(input);

    let mut r = HashSet::new();
    let mut p: HashSet<Node> = g.keys().cloned().collect();
    let mut x = HashSet::new();
    let mut cliques = Vec::new();
    bron_kerbosch1(&g, &mut r, &mut p, &mut x, &mut cliques);

    let max_clique_size = cliques.iter().map(|clique| clique.len()).max().unwrap_or(0);
    let max_clique = cliques
        .iter()
        .find(|clique| clique.len() == max_clique_size)
        .unwrap();

    let password = get_password(max_clique);
    Some(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("co,de,ka,ta")));
    }
}
