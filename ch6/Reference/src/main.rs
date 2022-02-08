fn main() {
    println!("Hello, world!");
    let s = String::from("hello");
    let mut s1 = String::from("world");
    let r1 = &s;
    let r2 = &s;//불변형은 두개 이상의 변수가 참조할 수 있다.
    println!("String is {}",r1);
    println!("String is {}",r2);
    let t1 = &mut s1;//가변형으로 참조는 불가능하다.
    // let t2 = &mut s1;//가변형은 하나의 변수만 참조할 수 있다. -> 데이터 레이스 방지
    //데이터 레이스는 서로 다른 스레드가 같은 값에 접근시 발생한다.
    // s1+=" Hi"; //이때 참조하고 있는 불변형 t1이 에러가 난다.
    println!("String is {}",t1);
    // println!("String is {}",t2);

    let str1 = String::from("Hello World");
    let hello = &str1[..5];//처음부터 4까지
    let world = &str1[6..];//6부터 끝까지
    let total = &str1[..];//참조하여 거의 복사된다.
    // let total1 = str1; 선언시 total1로 소유권이 이동하므로 주의
    println!("String = {}",hello);
    println!("String = {}",world);
    println!("String = {}",total);
    // println!("String = {}",total1);
    println!("String = {}",str1);
    println!("First word = {}",first_word(&str1));//문자열 참조
}
fn first_word(s: &String) -> &str{
    let bytes = s.as_bytes();//byte 배열로 변환
    for (i,&item) in bytes.iter().enumerate() {//반복문 (index,&element) 튜플을 이용
        if item == b' ' {//바이트 배열로 바꾼 bytes에 의해서 for문을 도는데 즉 ' '의 바이트와 같으면 
            return &s[..i];//처음부터 공백의 index 전까지 반환
        }
    }
    &s[..]//공백이 없었으면 전체를 하나의 문자열로 인식하여 반환
}//C언어와 다르게 Rus는 문자열을 UTF-8 포맷으로 저장하기 때문에 index로 접근 불가