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
			Temp::cold => 34,
		}
	}
	let temp = Temp::hot;
	println!("{}", calc_temp(temp));

}