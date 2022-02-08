#[derive(Debug)] //annotation을 추가함으로 구조체 자체를 출력이 가능하다.
struct Rectangle {
    width: u32,
    height: u32,
}//구조체 선언

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} squre pixels.",area(&rect)//rect 구조체 참조
    );
    println!("rect is {:#?}",rect);
    //{:#?}을 사용할 경우에는 필드 단위로 개행된다. like Json
    //{:?}을 사용할 경우에는 한 줄로 출력이 된다.
}

fn area(rectangle: &Rectangle) -> u32 {//구조체 참조 반환형은 u32 * u32 곱한값
    rectangle.width * rectangle.height
}