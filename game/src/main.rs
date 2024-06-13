use engine;

fn main() {
    if let Err(e) = engine::run("rray", "ilRECh") {
    // if let Err(e) = engine::run() {
        println!("{}", e.to_string());
    }
}
