pub struct Breakfast {//구조체 public 정의
    pub toast: String,//전체가 아닌 공개하고자 하는 필드만 public
    seasonal_fruit: String,
}

impl Breakfast {//구조체 메소드
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {//Toast값만 바꾼다
            toast: String::from(toast),//인자가 같으므로 생략 가능
            seasonal_fruit: String::from("peach"),
        }
    }
}

pub enum Appetizer {//구조체 public 접근
    Soup,
    Salad,
}

pub fn fix_incorrect_order() {
    cook_order();
    super::front_of_house::serving::serve_order();//정의된 모듈의 바깥의 함수에 접근 , 상대경로 앞에 super
    //왜냐하면 주문이 잘못되었는지 판한하기 위해서 해당하는 함수에 접근해야하기 때문에
}

fn cook_order() {println!("Cook Order");}