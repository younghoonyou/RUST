fn main() {
    let mut v = Vec::new();
    v.push(5);
    v.push(4);
    v.push(3);
    v.push(2);
    v.push(1);
    v.pop();
    if !v.is_empty() {
        println!("Not Empty");
    }
    v.rotate_left(1);
    v[1..4].rotate_right(2);
    v.reverse();
    // v.swap_remove(0);//큐 처럼 마지막 원소가 맨 앞으로 온다
    // v.swap_remove(0);
    // v.insert(0,6);// O(n)
    // v.remove(1);// O(n)
    let mut vec1 = vec![0; 5];//초기화 , 크기
    vec1.fill(1);
    for element in &mut v {//주소를 참조하므로 element는 포인터 역할
        *element += 10;
        print!("{} ",element);
    }
    println!();
    println!("Size = {}",v.len());
    for i in &mut vec1 {
        print!("{} ",i);
    }
    println!();
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        print!("{} ",top);
    }    
}
