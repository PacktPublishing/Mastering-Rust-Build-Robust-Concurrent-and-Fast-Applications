fn main(){
	let k = String::from("Hi");
	let b = String::from("!");
	let z = k + &b;
	println!("{}", z);
}