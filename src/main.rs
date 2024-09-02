// pva: persistent variables
mod pva;

fn main() {
    pva::new("config", false);
    println!("{:?}", pva::read_all("config"))
}
