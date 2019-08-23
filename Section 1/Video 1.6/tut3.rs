fn main(){
	let a = String::from("a");
	doesnt_take(&a);
	println!("{}", a);
}
fn doesnt_take(a: &String){
	println!("{}", a);
}