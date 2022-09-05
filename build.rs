use std::env::set_var;

fn main() {
	println!("LOADING ENVIROMENT VARIABLES!\n==============");
	let vars = include_str!(".env");
	vars.split("\n").for_each(|var| {
		let x = var.split("=").map(|x| x).collect::<Vec<&str>>();
        if x.len() > 1 {
            set_var(x[0], x[1].replace('"', ""));
        }
	})
}
