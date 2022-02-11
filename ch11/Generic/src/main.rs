struct Point<T, U> {//T U달라도 T끼리 같고 U끼리 같으면 된다.
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {//메소드 작성방법
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {//인스턴스 앞 부분 , 다른 Point 구조체 뒷부분
        Point {
            x: self.x,//
            y: other.y,//
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };//T는 정수 U는 실수
    let p2 = Point { x: "hello", y: 'c' };//T는 문자열 U는 문자형

    let p3 = p1.mixup(p2);

    println!("x: {}, y: {}", p3.x, p3.y);//5 , 'c' 출력 예상
}