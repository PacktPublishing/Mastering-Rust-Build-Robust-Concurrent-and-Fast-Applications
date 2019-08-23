fn main(){
	let mut a = vec![1,1,2];
	for var in &mut a {
		*var *= 0;
	}
	for var in &a {
		println!("{}", var);
	}
}