use std::collections::VecDeque;

fn bfs(start: usize, graph: &Vec<Vec<usize>>) {
    let mut visited = vec![false; graph.len()];
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited[start] = true;

    while let Some(node) = queue.pop_front() {
        println!("Visited {}", node);

        for &neighbor in &graph[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back(neighbor);
            }
        }
    }
}

fn main() {
    let graph = vec![
        vec![1, 2],    // 0
        vec![0, 3],    // 1
        vec![0, 3],    // 2
        vec![1, 2],    // 3
    ];

    bfs(0, &graph);
}