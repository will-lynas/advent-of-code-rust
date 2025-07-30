use gxhash::{
    HashMap,
    HashMapExt,
    HashSet,
};
use regex::Regex;

type Graph = HashMap<String, HashMap<String, usize>>;

pub fn parse(input: &str) -> Graph {
    let re = Regex::new(r"(\w+) to (\w+) = (\d+)").unwrap();
    let mut graph: Graph = HashMap::new();
    for line in input.lines() {
        let caps = re.captures(line).unwrap();
        let place1 = caps[1].to_string();
        let place2 = caps[2].to_string();
        let distance: usize = caps[3].parse().unwrap();
        graph
            .entry(place1.clone())
            .or_default()
            .insert(place2.clone(), distance);
        graph.entry(place2).or_default().insert(place1, distance);
    }
    graph
}

fn solve(graph: &Graph, remaining: &HashSet<&str>, distance: usize, current_place: &str) -> usize {
    if remaining.is_empty() {
        return distance;
    }
    remaining
        .iter()
        .map(|&new_place| {
            let mut new_remaining = remaining.clone();
            new_remaining.remove(new_place);
            solve(
                graph,
                &new_remaining,
                distance + graph[current_place][new_place],
                new_place,
            )
        })
        .min()
        .unwrap()
}

pub fn part1(graph: &Graph) -> usize {
    let places: HashSet<&str> = graph.keys().map(String::as_str).collect();
    places
        .iter()
        .map(|place| {
            let mut new_places = places.clone();
            new_places.remove(place);
            solve(graph, &new_places, 0, place)
        })
        .min()
        .unwrap()
}

pub fn part2(input: &Graph) -> usize {
    input.len()
}
