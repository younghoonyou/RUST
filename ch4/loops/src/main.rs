fn main() {
    let val = 3;
    loop_function(val);
    while_function(val);
    for_function();
}
fn loop_function(mut x: i32){
    let result = loop{
        x+=1;
        if x==10 {
            break x*2;
        }
    };
    println!("Result is {}",result);
}
fn while_function(mut x: i32){//while문은 조건에 따른 경우만 동작
    while x!=0 {//조건에 만족하지 않을 때 까지
        x-=1;
    }
    println!("Result is {}",x);
}
fn for_function(){// for num in min..max 할 경우 min부터 max까지 1씩 증가하며 대입
    let arr = [10,20,30,40,50];//(min..max).rev()를 통해서 숫자를 역순으로 참조 가능
    for element in arr.iter(){
        println!("Arr's Value is {}",element);
    }
    for element in (0..5).rev(){//MAX미만 고려해야한다.
        println!("Value is {}",arr[element]);
    }
}