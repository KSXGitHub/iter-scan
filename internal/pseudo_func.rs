/// Pseudo stateless function.
///
/// Unlike a function pointer, this forces static dispatching.
///
/// Unlike an `Fn` trait, this can be implemented to any type.
pub trait PseudoFunc<X, Y> {
    fn exec(x: X) -> Y;
}
