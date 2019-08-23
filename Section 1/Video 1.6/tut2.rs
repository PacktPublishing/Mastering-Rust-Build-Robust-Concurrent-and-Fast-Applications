fn main(){
	let a = String::from("a");
	takes(a);
	let a = gives();
	println!("{}", a);
}
fn takes(a: String){
	println!("{}", a);
}
fn gives() -> String{
	let a = String::from("b");
	a
}