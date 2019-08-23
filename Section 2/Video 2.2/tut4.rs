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
	impl Home{
		fn price(&self) -> i32{
			self.rooms * 100
		}
		fn bigger_than(&self, s: &Home) -> bool{
			self.rooms > s.rooms
		}
	}
	
	let h = make_home(String::from("my"), 12, true);
	let h2 = make_home(String::from("my"), 13, true);
	println!("{}", h.price());
	println!("{}", h.bigger_than(&h2));
}