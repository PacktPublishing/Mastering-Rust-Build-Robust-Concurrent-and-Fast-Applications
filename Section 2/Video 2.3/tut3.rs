fn main(){
	enum Temp{
		hot,
		normal,
		cold,
	}
	impl Temp {
		fn calc_temp(&self) -> i32{
			match self {
				&Temp::hot => 39,
				&Temp::normal => 36,
				_ => 34,
			}
		}
	}
	let temp = Temp::cold;
	println!("{}", temp.calc_temp());

}