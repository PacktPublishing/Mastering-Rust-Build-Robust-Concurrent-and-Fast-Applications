fn main(){
	struct Home (String, i32, bool);
	let home1 = Home(String::from("my"), 12, true);
	println!("rooms = {}", home1.1);
}