struct Person {
    name: String,
    age: u32,
}

fn main() {
    let people = vec![
        Person { name: "Alice".to_string(), age: 25 },
        Person { name: "Bob".to_string(), age: 17 },
        Person { name: "Charlie".to_string(), age: 30 },
    ];

    let adults: Vec<&Person> = people.iter()
        .filter(|p| p.age >= 18)
        .collect();

    for person in adults {
        println!("{} is an adult.", person.name);
    }
}

// 출력:
// Alice is an adult.
// Charlie is an adult.
//.iter() 사용: Person을 소유권 이동 없이 참조
//filter(|p| p.age >= 18): age >= 18인 사람만 남김
//.collect()는 Rust의 이터레이터 메서드 중 하나로, 
//이터레이터의 모든 요소를 수집하여 컬렉션(예: 벡터, 해시맵 등)으로 변환합니다.