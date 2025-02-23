fn is_valid_parentheses(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for ch in s.chars() {
        match ch {
            '(' | '{' | '[' => stack.push(ch),
            ')' => if stack.pop() != Some('(') { return false; },
            '}' => if stack.pop() != Some('{') { return false; },
            ']' => if stack.pop() != Some('[') { return false; },
            _ => continue,
        }
    }

    stack.is_empty() // 스택이 비어있으면 괄호가 모두 짝이 맞음
}

fn main() {
    let test1 = "({[]})";
    let test2 = "({[})";
    println!("{}: {}", test1, is_valid_parentheses(test1)); // true
    println!("{}: {}", test2, is_valid_parentheses(test2)); // false
}

// 상황 1: 괄호 짝 맞추기 (Parentheses Matching)설명: 수식이나 코드에서 괄호가 올바르게 열리고 닫혔는지 확인할 때 스택이 유용합니다. 
//열린 괄호((, {, [)를 스택에 넣고, 닫힌 괄호가 나오면 스택에서 꺼내 짝이 맞는지 확인합니다.
//활용 사례: 컴파일러, 텍스트 에디터의 구문 검사 등.