// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

// I AM NOT DONE

// #[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    // nao é possivel fazer varios borrow mutaveis ao mesmo tempo e seguidos
    // sem que o primeiro seja liberado ou utilizado de alguma forma como a nova variavel
    assert_eq!(x, 1200);
}
