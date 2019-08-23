fn main(){
	let s = String::from("b");
	{
		let s = String::from("a");
	}
	println!("{}", s);
}