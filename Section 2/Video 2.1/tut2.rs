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
	let mut home2 = Home {
		sold: false,
		..home1
	};
	println!("sold = {}", home2.sold);
	println!("rooms = {}", home2.rooms);
}