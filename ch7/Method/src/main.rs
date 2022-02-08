struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    let tmp = Rectangle::new(10,10);//인스턴스 선언과 동시에 변수 설정

    println!(
        "The area of the rectangle is {} square pixels.",
        tmp.area()
    );
}

impl Rectangle {//impl 메소드 선언시 필요 &mut self도 가능
    fn area(&self) -> u32 {//첫번째 매개변수는 self를 인자로 한다.
        self.width * self.height
    }
    fn new(width: u32,height: u32) -> Rectangle {
        Rectangle {
            width,//인자와 같으면 생략 가능
            height,
        }
    }
}