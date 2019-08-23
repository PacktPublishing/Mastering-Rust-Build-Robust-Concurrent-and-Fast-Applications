fn main(){
	let s = 32;
	{
		let s = 23;
	}
	println!("{}", s);
}