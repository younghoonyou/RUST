fn main() {
    let first = 35;
    let second = 46;
    println!("Main call");
    call();
    parameter(first,second);

    let x = 5;
    let y = {
        let x = 3;//위의 x와는 다른 변수 Shadowing이 적용되지 않는다.
        x + 1//return 값에는 세미콜론 없음
    };
    println!("X:{}",x);
    println!("Y:{}",y);
    println!("Function value is {}",plus_one(x));//x+1값을 출력
}
fn call(){
    println!("Function call");
}
fn parameter(x:i32,y:i32){
    println!("X Value is {}",x);
    println!("Y Value is {}",y);
}
fn plus_one(x:i32)->i32{//앞의 i32은 x의 자료형을 뒤의 i32는 반환형을 나타낸다.
    x+1
}//반환형이 없을 경우 표시하지 않는다.