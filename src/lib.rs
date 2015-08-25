#![feature(test)]

extern crate rand;

use self::rand::distributions::{Range, IndependentSample};


enum ArithmeticEnum {
    Zero,
    Dot{a: f64, b: f64},
    SqrDot{a: f64, b: f64}
}

impl ArithmeticEnum {
    fn apply(&self, x: f64, y: f64) -> f64 {
        match self {
            &ArithmeticEnum::Zero => 0.0,
            &ArithmeticEnum::Dot{a: a, b: b} => a * x + b * y,
            &ArithmeticEnum::SqrDot{a: a, b: b} => a * x*x + b * y*y,
        }
    }
}


trait ArithmeticTrait {
    fn apply(&self, x: f64, y: f64) -> f64;
}

struct Zero;

struct Dot {
    a: f64,
    b: f64
}

struct SqrDot {
    a: f64,
    b: f64
}

impl ArithmeticTrait for Zero {
    fn apply(&self, x: f64, y: f64) -> f64 {
        0.0
    }
}

impl ArithmeticTrait for Dot {
    fn apply(&self, x: f64, y: f64) -> f64 {
        self.a * x + self.b * y
    }
}

impl ArithmeticTrait for SqrDot {
    fn apply(&self, x: f64, y: f64) -> f64 {
        self.a * x*x + self.b * y*y
    }
}



extern crate test;

#[test]
fn test_enum_variant() {
    assert_eq!(ArithmeticEnum::Zero.apply(2.0, 5.0), 0.0);
    assert_eq!(ArithmeticEnum::Dot{a: 3.0, b: 7.0}.apply(2.0, 5.0), 41.0);
    assert_eq!(ArithmeticEnum::SqrDot{a: 3.0, b: 7.0}.apply(2.0, 5.0), 187.0);
}


#[test]
fn test_trait_variant() {
    assert_eq!(Zero.apply(2.0, 5.0), 0.0);
    assert_eq!(Dot{a: 3.0, b: 7.0}.apply(2.0, 5.0), 41.0);
    assert_eq!(SqrDot{a: 3.0, b: 7.0}.apply(2.0, 5.0), 187.0);
}


#[bench]
fn bench_enum_variant(bench: &mut test::Bencher) {
	let n = 1024;
    let r = 32;

	let val_rng: Range<f64> = Range::new(-10.0, 10.0);
	let mut rng = rand::thread_rng();

    let mut objects = Vec::with_capacity(n*3);
    let mut values = Vec::with_capacity(n*6);

    for _ in 0..n {
        objects.push(ArithmeticEnum::Zero);
        objects.push(ArithmeticEnum::Dot{a: val_rng.ind_sample(&mut rng), b: val_rng.ind_sample(&mut rng)});
        objects.push(ArithmeticEnum::SqrDot{a: val_rng.ind_sample(&mut rng), b: val_rng.ind_sample(&mut rng)});
    }

    for _ in 0..n*6 {
        values.push(val_rng.ind_sample(&mut rng));
    }

    bench.iter(|| {
        let mut x = 0.0;
    	for _ in 0..r {
            for i in 0..objects.len() {
                x += objects[i].apply(values[i*2], values[i*2+1]);
            }
		}
	});
}

#[bench]
fn bench_trait_variant(bench: &mut test::Bencher) {
	let n = 1024;
    let r = 32;

	let val_rng: Range<f64> = Range::new(-10.0, 10.0);
	let mut rng = rand::thread_rng();

    let mut zeros = Vec::with_capacity(n);
    let mut dots = Vec::with_capacity(n);
    let mut sqrdots = Vec::with_capacity(n);
    let mut objects: Vec<&ArithmeticTrait> = Vec::with_capacity(n*3);
    let mut values = Vec::with_capacity(n*6);

    for _ in 0..n {
        zeros.push(Zero);
        dots.push(Dot{a: val_rng.ind_sample(&mut rng), b: val_rng.ind_sample(&mut rng)});
        sqrdots.push(SqrDot{a: val_rng.ind_sample(&mut rng), b: val_rng.ind_sample(&mut rng)});
    }

    for triple in zeros.iter().zip(dots.iter()).zip(sqrdots.iter()) {
        let ((a, b), c) = triple;
        objects.push(a);
        objects.push(b);
        objects.push(c);
    }

    for _ in 0..n*6 {
        values.push(val_rng.ind_sample(&mut rng));
    }

    bench.iter(|| {
        let mut x = 0.0;
    	for _ in 0..r {
            for i in 0..objects.len() {
                x += objects[i].apply(values[i*2], values[i*2+1]);
            }
		}
	});
}
