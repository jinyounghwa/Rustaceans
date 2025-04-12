// 숫자중에서 중복이 없는 수를 출력
// use std::collections::HashSet;

// fn main() {
//     let nums = vec![1, 2, 2, 3, 4, 4, 5];
//     let unique: HashSet<_> = nums.into_iter().collect();

//     println!("{:?}", unique); // {1, 2, 3, 4, 5}
// }


// BTreeSet은 정렬된 상태로 저장되므로, 중복된 값은 자동으로 제거됩니다.
//use std::collections::BTreeSet;

// fn main() {
//     let scores = vec![50, 70, 90, 90, 80];
//     let sorted_scores: BTreeSet<_> = scores.into_iter().collect();

//     for score in &sorted_scores {
//         println!("{}", score);
//     }
//     // 출력: 50, 70, 80, 90
// }
//그래프에서 최단 경로를 찾기 위해 BFS를 구현하라."
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
