fn main() {
    let mut x = 3;//변수에 mut을 함으로 가변형이 된다.
    println!("Number is {}",x);
    x = 5;
    println!("Number is {}",x);
    let tup: (i32,f64,u8) = (500,5.43,1);
    println!("Value is {}",tup.0);
    println!("Value is {}",tup.1);
    println!("Value is {}",tup.2);
}
//하지만 우리가 알듯이 const형을 사용하면 완전한 불가변형이다.
//약간의 변수 오버로딩 처럼 같은 변수명을 선언하면 mut를 선언하지 않아도 변수값이 변한다.