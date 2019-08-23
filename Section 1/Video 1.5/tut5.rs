fn main(){
	let s = String::from("Hello");
	let a = &s[1..2];
	let b = &s[..2];
	let c = &s[1..];
	println!("{}", a);
	println!("{}", b);
	println!("{}", c);
}