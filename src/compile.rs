use super::*;

pub struct ConsDrawPass<A, B> {
    a: A,
    b: B,
}

impl<A, B> DrawPass<Singular> for ConsDrawPass<A, B>
where
    A: DrawPass<Beginning>,
    B: DrawPass<End>,
{
    #[inline]
    fn render(&self) {
        self.a.render();
        self.b.render();
    }
}

impl<A, B> DrawPass<End> for ConsDrawPass<A, B>
where
    A: DrawPass<Middle>,
    B: DrawPass<End>,
{
    #[inline]
    fn render(&self) {
        self.a.render();
        self.b.render();
    }
}

macro_rules! consdrawpass {
    ($x:expr) => { $x };
    ($x:expr, $($y:expr),+) => { ConsDrawPass { a: $x, b: consdrawpass!($($y),*)} };
}

#[cfg(test)]
mod bench {
    extern crate test;

    use super::*;
    use test::*;

    #[bench]
    fn compiled_bench(bencher: &mut Bencher) {
        // let dp = ConsDrawPass {a: One, b: ConsDrawPass {a: Two, b: ConsDrawPass { a: Three, b: Four }}};
        let dp = consdrawpass!(One, Two, Three, Four);
        bencher.iter(|| {
            DrawPass::<Singular>::render(&dp);
        });
    }
}