use super::*;

// This assumes there's always at least 2 elements
// If we were implementing this properly, we'd probably do it much better.
struct ManyDrawPasses (Box<dyn DrawPass<Beginning>>, Vec<Box<dyn DrawPass<Middle>>>, Box<dyn DrawPass<End>>);
impl DrawPass<Singular> for ManyDrawPasses {
    fn render(&self) {
        self.0.render();
        for dp in self.1.iter() {
            dp.render();
        }
        self.2.render();
    }
}


#[cfg(test)]
mod bench {
    extern crate test;

    use super::*;
    use test::*;

    #[bench]
    fn runtime_bench(bencher: &mut Bencher) {
        let dp = ManyDrawPasses (Box::new(One), vec![Box::new(Two), Box::new(Three)], Box::new(Four));
        bencher.iter(|| {
            DrawPass::<Singular>::render(&dp);
        });
    }
}