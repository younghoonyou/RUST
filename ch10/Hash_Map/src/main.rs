use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);
    scores.entry(String::from("Gray")).or_insert(50);//entry로 검새하고 없으면 추가
    let tmp = "Blue".to_string();
    scores.remove(&tmp);//해시맵 삭제
    for i in scores {
        println!("{:?}",i);
    }
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);//name과 value소유권은 없어진다.

}
