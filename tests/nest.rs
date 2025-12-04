// #![feature(generic_const_exprs)]
// #![recursion_limit = "256"]

// #[test]
// fn benchmark_type_inference_vs_explicit() {
//     use std::time::Instant;

//     macro_rules! deep_nested {
//         ($n:expr) => {{
//             let start = Instant::now();
//             let _value = deep_inferred!($n);
//             let inferred_dur = start.elapsed();

//             let start = Instant::now();
//             let _value: DeepExplicit<$n> = deep_explicit!($n);
//             let explicit_dur = start.elapsed();

//             println!(
//                 "Depth {:>3}: inferred {:>8} µs | explicit {:>8} µs | ratio {:.2}x",
//                 $n,
//                 inferred_dur.as_micros(),
//                 explicit_dur.as_micros(),
//                 inferred_dur.as_micros() as f64 / explicit_dur.as_micros().max(1) as f64
//             );
//         }};
//     }

//     // Generate deeply nested types via macros
//     deep_nested!(10);
//     deep_nested!(20);
//     deep_nested!(30);
//     deep_nested!(40);
//     deep_nested!(50);
// }

// // Fully inferred version (forces heavy type inference)
// macro_rules! deep_inferred {
//     (0) => {
//         ()
//     };
//     ($n:expr) => {
//         (deep_inferred!($n - 1), $n)
//     };
// }

// // Explicit generic type
// macro_rules! deep_explicit {
//     (0) => {
//         ()
//     };
//     ($n:expr) => {
//         (DeepExplicit::<{ $n - 1 }>::T, $n)
//     };
// }

// trait Assoc {
//     type T;
// }
// enum DeepExplicit<const N: usize> {
//     Phantom(std::marker::PhantomData<()>),
// }
// impl Assoc for DeepExplicit<0> {
//     type T = ();
// }
// impl<const N: usize> Assoc for DeepExplicit<N> {
//     type T = (<DeepExplicit<{ N - 1 }> as Assoc>::T, usize);
// }
