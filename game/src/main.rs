use engine;

fn main() {
    if let Err(e) = engine::run("rray", "ilRECh") {
        println!("{}", e.to_string());
    }
}
