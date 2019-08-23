fn main(){
	let a = "hi";
	let mut b = a.to_string();
	b.push('!');
	b.push_str(" Hello!");
	println!("{}", b);
}