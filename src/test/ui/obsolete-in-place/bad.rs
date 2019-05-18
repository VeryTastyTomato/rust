// Check that `<-` and `in` syntax gets a hard error.

fn main() {
    let (x, y, foo, bar) = (0, 0, 0, 0);
    x <- y; // ok: parses as a comparison
    in(foo) { bar }; //~ERROR expected expression, found keyword `in`
}
