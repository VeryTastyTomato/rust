// run-pass

fn main() {
    let x = -5;
    if x<-1 { // ok: parses as a comparison
        println!("ok");
    }
}
