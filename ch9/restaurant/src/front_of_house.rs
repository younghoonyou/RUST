//크레이트의 모듈 시작부분
pub mod hosting {//모듈 안의 모듈 , public 모듈
    pub fn add_to_waitlist() {println!("Add Wait list");}//public 함수
    fn seat_at_table() {println!("Seat Table");}
}

pub mod serving {// 모듈
    fn take_order() {println!("Take Order");}
    pub fn serve_order() {println!("Serve Order");}
    fn take_payment() {println!("Take Payment");}
}