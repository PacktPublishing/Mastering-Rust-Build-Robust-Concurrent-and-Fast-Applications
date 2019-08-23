fn main(){
	let a = vec![1,1,2];
	match a.get(2){
		Some(var) => print!("{}", var),
		None => println!("not defined"),
	}
}