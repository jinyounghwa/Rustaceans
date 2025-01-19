trait Shape { 
    fn area(&self) -> f64; //  (각자별로 만들어낼 때 필요하다면, 함수를 추가해도 좋습니다)
}


// 'Rectangle' object: Rectangle 형태를 나타내는 객체. 이에 대한 구체적인 코드 예시와 설명을 먼저 작성하는 것을 추천합니다.

struct Rectangle { 
    width: f64,  
    height: f64 
} // 각 객체가 다르게 사용될 때 필요한 정보를 정의해주세요.


impl Shape for Rectangle { //  'Shape' trait을 구현하는 데에 도움이 되는 방법을 제시합니다.

    fn area(&self) -> f64 {
        // 넓이의 계산 및 return 값을 정의하세요
        self.width * self.height   
    }
}

fn main() {
    let rect = Rectangle {
        width: 10.0,
        height: 5.0,
    };
    println!("Rectangle area: {}", rect.area());
}