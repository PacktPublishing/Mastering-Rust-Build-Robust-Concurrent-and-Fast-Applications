fn main(){
	let o = '+';
	let a = 5;
	let b = 6;
	calc(a,o,b);
	let o = '-';
	calc(a,o,b);
	let o = '*';
	calc(a,o,b);
	let o = '/';
	calc(a,o,b);
}
fn calc(x: i32, o: char, y: i32){
	match o{
		'+' => println!("{}", add(x,y)),
		'-' => println!("{}", sub(x,y)),
		'*' => println!("{}", mul(x,y)),
		'/' => println!("{}", div(x,y)),
		_ => println!("error")
	}
}
fn add(x: i32, y: i32) -> i32{
	x+y
}
fn sub(x: i32, y: i32) -> i32{
	x-y
}
fn mul(x: i32, y: i32) -> i32{
	x*y
}
fn div(x: i32, y: i32) -> i32{
	x/y
}