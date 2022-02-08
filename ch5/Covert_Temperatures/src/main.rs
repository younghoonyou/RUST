use std::io;

fn main() {
    println!("[1] Convert Fahrenheit to Celsius");
    println!("[2] Convert Celsius to Fahrenheit");
    let choice = loop{
        println!("Input Choice Data");
        let mut choice = String::new();
        io::stdin().read_line(&mut choice)//&mut 변수를 기억하자
        .expect("Fail to Read line");
        
        let choice: u32 = match choice.trim().parse(){//공백으로 자르기
            Ok(num) => num,
            Err(_) =>{
                println!("Input 1 or 2");
                continue
            },
        };
        if choice==1 || choice==2 {//조건 설정
            break choice;
        }
    };
    let temp = loop {
        println!("Input temp Data");
        let mut temp = String::new();//mut 가변성 필수
        io::stdin().read_line(&mut temp)
        .expect("Fail to Read line");

        let temp: f64 = match temp.trim().parse() {//부동 소수점으로 casting
            Ok(num) => num,
            Err(_) => {
                println!("Please input the tempeture without unit symbol");
                continue//에러에 따른 return 이므로 ; 생략
            },
        };
        break temp;//변수 설정에대한 loop break
    };
    let temp = if choice == 1 {//불가변성으로 shawding
        fa_to_cel(temp)//1을 선택할 경우 자동으로 temp에 값이 저장    return 이므로 ; 없음 조심
    }
    else{
        cel_to_fa(temp)//2를 선택할 경우 자동으로 temp에 값이 저장    return 이므로 ; 없음 조심
    };

    println!("Result : {}",temp);
}

fn fa_to_cel(x: f64) -> f64 {
    (x - 32.) / 1.8
}
fn cel_to_fa(x: f64)->f64{
    x * 1.8 + 32.//32.0으로 초기화하여  정수가 아닌 부동소수점으로 계산
}
//°F = °C × 1.8 + 32
//°C = (°F − 32) / 1.8 부동소수점이니 자료형은 f64로 선정하자