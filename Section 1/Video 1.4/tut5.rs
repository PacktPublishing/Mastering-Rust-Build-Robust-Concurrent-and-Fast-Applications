fn main(){
	func();
	println!("{}", b);
}
fn func(){
	let b = String::from("c");
	println!("{}", b);
}