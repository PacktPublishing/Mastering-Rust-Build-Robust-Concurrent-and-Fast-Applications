fn main(){
	enum Temp{
		hot,
		normal,
		cold,
	}
	fn calc_temp(t: Temp) -> i32{
		match t {
			Temp::hot => 39,
			Temp::normal => 36,
			_ => 34,
		}
	}
	let temp = Temp::cold;
	println!("{}", calc_temp(temp));

}