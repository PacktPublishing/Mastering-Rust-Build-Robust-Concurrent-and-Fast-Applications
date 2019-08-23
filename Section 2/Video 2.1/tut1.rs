fn main(){
	struct Home {
    	name: String,
    	rooms: i32,
    	sold: bool,
	}
	let mut home1 = Home {
    	name: String::from("myhome"),
    	rooms: 13,
    	sold: false,
	};
	home1.sold = true;
	println!("sold = {}", home1.sold);
}