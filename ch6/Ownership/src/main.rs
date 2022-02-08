fn main() {
    let mut s = String::from("Hello");
    s.push_str(" world");//문자열 뒤에 추가
    s+=" Good";//문자열 뒤에 추가
    println!("{}",s);
    let s = {//소유권 규칙
        let tmp = String::from("hello");//대괄호를 나가면 tmp 자동으로 할당해제
        tmp
    };
    println!("{}",s);
    //Copy 
    let x = 5;
    let y = x;//스택 메모리에 있는 값을 bind 할 경우 값이 복사가 된다.
    println!("Value is {}",x);
    println!("Value is {}",y);
    let s1 = String::from("good");
    let s2 = s1;
    // println!("Word is {}",s1); 빌드시 에러가 발생한다. because 스택 메모리가 아닌
    //힙 메모리에 할당된 변수룰 bind 할 경우 소유권이 s1에서 s2로 이전된다.
    //단 정수 자료형은 Copy 트레이트를 가지고 있다.
    println!("Word is {}",s2);
    let s1 = s2.clone();//소유권이 이전 되지 않으며 값이 저장되어 있는 힙메모리 자체를 복사한다.
    //그만큼 오베헤드가 발생한다.
    println!("Word is {}",s1);
}
