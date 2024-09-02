// pva: persistent variables
mod pva;

fn main() {
    pva::new("pva/config", false);
    pva::write("pva/config", "sdf", "ab3");
    pva::write("pva/config", "abc", "123");
    pva::write("pva/config", "ert", "12d");
    println!("{}", pva::read("pva/config", "abc"));
    pva::remove("pva/config", "abc");
}
