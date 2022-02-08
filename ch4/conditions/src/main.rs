fn main() {
    let condition = true;
    let number = if condition {//조건에 따른 변수 값 띄어쓰기를 조심하자 RUST 특성
        5
    }
    else{//대괄호 필수적
        6
    };
    println!("Value is {}",number);
}
