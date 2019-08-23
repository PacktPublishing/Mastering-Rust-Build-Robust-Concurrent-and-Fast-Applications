fn main(){
	struct Home {
    	name: String,
    	rooms: i32,
    	sold: bool,
	}
	fn make_home(n: String, r: i32, s: bool)->Home{
		Home {
			name: n,
			rooms: r,
			sold: s,
		}
	}
	fn price(home: &Home) -> i32{
		home.rooms * 100
	}
	let h = make_home(String::from("my"), 12, true);
	println!("{}", price(&h));
}