use itertools::Itertools;

advent_of_code::solution!(23);

#[derive(Debug)]
struct Graph {
    nodes: Vec<Node>,
}

impl Graph {
    fn try_add_node(&mut self, label: &str) {
        if !self.nodes.iter().any(|node| node.label == label) {
            self.nodes.push(Node {
                label: label.to_string(),
                neighbors: vec![],
            });
        }
    }

    fn get_node(&self, label: &str) -> Option<&Node> {
        self.nodes.iter().find(|node| node.label == label)
    }

    fn get_node_mut(&mut self, label: &str) -> Option<&mut Node> {
        self.nodes.iter_mut().find(|node| node.label == label)
    }

    fn find_all_triangles(&self) -> Vec<Vec<String>> {
        let mut triangles = vec![];
        for node in self.nodes.iter() {
            for neighbor1 in node.neighbors.iter() {
                let neighbor1 = self.get_node(neighbor1).unwrap();
                for neighbor2 in neighbor1.neighbors.iter() {
                    let neighbor2 = self.get_node(neighbor2).unwrap();
                    if neighbor2.has_neighbor(&node.label) {
                        let mut triangle = vec![
                            node.label.to_string(),
                            neighbor1.label.to_string(),
                            neighbor2.label.to_string(),
                        ];
                        triangle.sort();

                        if !triangles.contains(&triangle) {
                            triangles.push(triangle);
                        }
                    }
                }
            }
        }
        triangles
    }
}

#[derive(Debug)]
struct Node {
    label: String,
    neighbors: Vec<String>,
}

impl Node {
    fn try_add_neighbor(&mut self, neighbor: &str) {
        if !self.has_neighbor(neighbor) {
            self.neighbors.push(neighbor.to_string());
        }
    }

    fn has_neighbor(&self, neighbor: &str) -> bool {
        self.neighbors.iter().any(|node| *node == neighbor)
    }
}

fn create_graph(input: &str) -> Graph {
    let mut graph = Graph { nodes: vec![] };
    for line in input.lines() {
        let (n1, n2) = line.split_once("-").unwrap();
        graph.try_add_node(n1);
        graph.try_add_node(n2);

        if let Some(node1) = graph.get_node_mut(n1) {
            node1.try_add_neighbor(n2);
        }

        if let Some(node2) = graph.get_node_mut(n2) {
            node2.try_add_neighbor(n1);
        }
    }
    graph
}

pub fn part_one(input: &str) -> Option<u32> {
    let g = create_graph(input);
    let triangles = g.find_all_triangles();
    let triangles_with_t_nodes: Vec<_> = triangles
        .iter()
        .filter(|tri| tri.iter().any(|node| node.starts_with('t')))
        .collect();
    Some(triangles_with_t_nodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        assert_eq!(result, None);
    }
}
