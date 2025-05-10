struct Company {
    name:String,
    ceo:Option<String>,
}
impl Company {
    fn new(name:&str, ceo:&str) -> Self{
        let ceo = match ceo {
            "" => None,
            ceo => Some(ceo.to_string())
        };
        Self {
            name: name.to_string(),
            ceo,
        }
    }
    fn get_ceo(&self) -> Option<String> {
        self.ceo.clone()
    }
}
fn main(){
    let company_vec = vec![
        Company::new("Google", "Sundar"),
        Company::new("Apple", "Tim"),
        Company::new("Microsoft", "Satya"),
    ];
    let all_the_ceos = company_vec
    .into_iter()
    .filter_map(|company| company.get_ceo())
    .collect::<Vec<_>>();
    println!("All the ceos: {:?}", all_the_ceos);
}
/**
 *작동 방식은 다음과 같습니다:

into_iter()로 회사 벡터를 소유권 이전 반복자로 변환
filter_map()에 클로저를 전달하여 각 회사에서 get_ceo() 메서드 호출
get_ceo()는 Option을 반환:
CEO가 있으면 Some(String) 반환
CEO가 없으면 None 반환
filter_map은 자동으로:
Some 값은 내부 값을 추출하여 유지
None 값은 결과에서 제외
마지막으로 collect::<Vec<_>>()로 결과를 벡터로 수집
이 예제에서는 모든 회사에 CEO가 있지만, 만약 Company::new("회사명", "")처럼 빈 문자열로 CEO를 생성했다면 해당 회사의 CEO는 결과 벡터에 포함되지 않았을 것입니다.

filter_map은 Option을 반환하는 함수와 함께 사용할 때 특히 유용하며, filter와 map을 별도로 호출하는 것보다 더 간결하고 효율적인 코드를 작성할 수 있게 해줍니다. 
 */