fn main(){
	let mut a = String::from("a");
	doesnt_take(&mut a);
	println!("{}", a);
}
fn doesnt_take(a: &mut String){
	a.push_str("b");
}