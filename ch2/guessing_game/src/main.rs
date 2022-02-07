use std::io;//input output library
use rand::Rng;
use std::cmp::Ordering;
fn main() {
	let secret = rand::thread_rng().gen_range(1,101);
	println!("Guess the number!");
	loop{
	println!("Please input your guess.");
	let mut guess = String::new();//mut를 통해서 값을 변경가능 
	io::stdin()//입력을 받는다
		.read_line(&mut guess)//입력 메소드 사용자가 입력한 값을 얻는다
		.expect("Fail to read line");//메서드 Error일 경우 출력
	let guess :u32 = match guess.trim().parse(){//공백을 제거하고 secret과 같은 문자형으로 캐스팅
		Ok(num) => num,//정상적으로 동작했으면
		Err(_) => continue,//에러면
	};
	println!("You guessed: {}",guess);//출력
	match guess.cmp(&secret){//각각의 비교군
		Ordering::Less => println!("Small"),
		Ordering::Greater => println!("Big"),
		Ordering::Equal => {
			println!("Answer!!");
			break;
		},
	}
	}
	// println!("Number is {}",secret);
}//toml파일에서 Rand 크레이트를 추가함으로 빌드시 라이브러리 다운이 가능하다
