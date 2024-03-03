pub mod app;
pub mod ui;
pub mod term;

fn main() {
    term::Term::new().unwrap().run().unwrap();
}
