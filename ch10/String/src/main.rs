use std::io;
fn main() {
    let mut st  = String::new();//String형태
    st+="Hoon";
    let mut strr = "ABCD".to_string();//&str이 아닌 String형태
    println!("{}",st);
    let mut s = String::new();
    io::stdin().read_line(&mut s)
    .expect("Fail");
    let string_split = s.split(' ');//문자열 공백 자르기
    for i in string_split {
        println!("{} ",i);
    }
}
//일단 문자열이 제일 어려울듯하다 왜냐면 index 방식이 아니라 바이트 방식이기에ㅠㅠ
//열공해야겠다.
//&str은 읽기전용 String은 수정가능하다