use itertools::Itertools;
use std::collections::{BTreeSet, HashMap, HashSet};

advent_of_code::solution!(23);

type Edges = Vec<(String, String)>;
type AdjacencyMap = HashMap<String, HashSet<String>>;

pub fn part_one(input: &str) -> Option<u32> {
    let (edges, adjacency) = parse(input);

    let triplets: BTreeSet<_> = edges
        .iter()
        .flat_map(|(a, b)| {
            adjacency[a]
                .intersection(&adjacency[b])
                .filter(|c| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
                .map(|c| BTreeSet::from_iter([a.clone(), b.clone(), c.clone()]))
        })
        .collect();

    Some(triplets.len() as u32)
}

fn parse(input: &str) -> (Edges, AdjacencyMap) {
    let mut adjacency: HashMap<String, HashSet<String>> = HashMap::new();
    let edges: Vec<(String, String)> = input
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            (a.to_string(), b.to_string())
        })
        .collect();

    edges.iter().for_each(|(a, b)| {
        adjacency.entry(a.clone()).or_default().insert(b.clone());
        adjacency.entry(b.clone()).or_default().insert(a.clone());
    });

    (edges, adjacency)
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, adjacency) = parse(input);

    Some(find_max_clique(&adjacency).iter().sorted().join(","))
}

/*
algorithm BronKerbosch2(R, P, X) is
    if P and X are both empty then
        report R as a maximal clique
    choose a pivot vertex u in P ⋃ X
    for each vertex v in P \ N(u) do
        BronKerbosch2(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
        P := P \ {v}
        X := X ⋃ {v}
 */
fn bron_kerbosch(
    graph: &AdjacencyMap,
    r: &mut BTreeSet<String>,
    p: &mut HashSet<String>,
    x: &mut HashSet<String>,
    max_clique: &mut BTreeSet<String>,
) {
    if p.is_empty() && x.is_empty() {
        if r.len() > max_clique.len() {
            *max_clique = r.clone();
        }
        return;
    }

    let u = p.union(x).next().unwrap().clone();
    let neighbors: HashSet<_> = graph.get(&u).unwrap().iter().cloned().collect();

    let p_copy = p.clone();
    for v in p_copy.difference(&neighbors) {
        let v = v.clone();
        r.insert(v.clone());
        let v_neighbors: HashSet<_> = graph.get(&v).unwrap().iter().cloned().collect();

        let mut new_p: HashSet<_> = p.intersection(&v_neighbors).cloned().collect();
        let mut new_x: HashSet<_> = x.intersection(&v_neighbors).cloned().collect();

        bron_kerbosch(graph, r, &mut new_p, &mut new_x, max_clique);

        r.remove(&v);
        p.remove(&v);
        x.insert(v);
    }
}

pub fn find_max_clique(adjacency: &HashMap<String, HashSet<String>>) -> BTreeSet<String> {
    let mut r = BTreeSet::new();
    let mut p: HashSet<_> = adjacency.keys().cloned().collect();
    let mut x = HashSet::new();
    let mut max_clique = BTreeSet::new();

    bron_kerbosch(adjacency, &mut r, &mut p, &mut x, &mut max_clique);
    max_clique
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
