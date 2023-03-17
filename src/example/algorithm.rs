use rand::{
    distributions::{Distribution, Standard, Alphanumeric},
    thread_rng, Rng,
};

pub fn create_rng<T: std::fmt::Debug>() -> T
where
    Standard: Distribution<T>,
{
    let mut rng = thread_rng();
    let n = rng.gen::<T>();
    n
}

pub fn create_rng_scope<
    T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform + std::fmt::Debug,
>(
    min: T,
    max: T,
) -> T {
    let mut rng = thread_rng();
    let n = rng.gen_range(min..max);
    n
}

#[derive(Debug)]
pub struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Distribution<Point> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let (rand_x, rand_y, rand_z) = rng.gen();
        Point {
            x: rand_x,
            y: rand_y,
            z: rand_z,
        }
    }
}

pub fn create_rng_point() -> Point {
    let mut rng = thread_rng();
    rng.gen::<Point>()
}


pub fn create_rng_str(len:i32) -> String {
    (0..len).map(|_| thread_rng().sample(Alphanumeric) as char).collect()
}


