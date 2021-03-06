/* It prints:
NaN NaN*/
fn main() {
    fn sqrt() {}
    trait HasSquareRoot {
        fn sqrt(self) -> Self;
    }
    impl HasSquareRoot for f32 {
        fn sqrt(self) -> Self { f32::sqrt(self) }
    }
    impl HasSquareRoot for f64 {
        fn sqrt(self) -> Self { f64::sqrt(self) }
    }
    fn quartic_root<Number>(x: Number) -> Number
    where Number: HasSquareRoot {
        x.sqrt().sqrt()
    }
    sqrt();
    print!("{} {}",
        quartic_root(-100f64),
        quartic_root(-100f32));
}
