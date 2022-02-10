#[no_mangle]//어노테이션을 추가함으로 맹글링 스위치 꺼짐 다른 언어에서 동적 라이브러리 사용 가능
pub mod front_of_house;//모듈 불러오기
pub mod back_of_house;

use crate::front_of_house::hosting;//절대경로로 받아오면 코드를 간단히 hosting으로 접근 할 수 있다.
//use self::front_of_house::hosting//상대경로로 받아올 경우
pub extern fn eat_at_restaurant() {//public
    // crate::front_of_house::hosting::add_to_waitlist();//절대경로 인한 함수호출
    // front_of_house::hosting::add_to_waitlist();//상대경로로 인한 함수 호출
    hosting::add_to_waitlist();//위의 두 경우와 같은 선언 use를 사용함으로 더 간소화
    let appetizer = back_of_house::Appetizer::Salad;
    let mut meal = back_of_house::Breakfast::summer("Bacon");

    meal.toast = String::from("Wheat");//가변형이므로 바뀐다.
    println!("I'd like to eat {}",meal.toast);
}
//target/debug/안에 rlib 즉 정적 라이브러리가 생성된 것을 볼 수 있다.
//동적 라이브러리로 전환하고 싶을 때 [lib]을 toml에 추가하자