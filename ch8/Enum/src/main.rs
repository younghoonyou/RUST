// enum Option<T> {//열거형  <T>는 제네릭
//     Some(T),
//     None,
// }
//열거형 사용시 Option::Some을 해야하지만 기본으로 포함되어 있다.
fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("five : {}",print_option(five));
    println!("six : {}",print_option(six));
    println!("none : {}",print_option(none));
    // let some_u8_value = Some(0u8);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     _=>(),//Some에 대한 식만 적용하고 나머지는 동일
    // }
    // println!("value : {}",some_u8_value);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {//match 연산자를 통해서 Some인 경우만 연산 수행가능
        None => None,//None은 None
        Some(i) => Some(i+1),//값 + 1
    }
}

fn print_option(x: Option<i32>) -> String {
    match x {//match는 흐름 제어 연산자
        None => String::from("None"),
        Some(i) => format!("{}",i),
    }
}