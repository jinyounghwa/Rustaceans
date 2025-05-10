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