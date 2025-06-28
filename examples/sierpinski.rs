use caixas::{Box, Horizontal, Vertical};

/// Creates a Sierpinski triangle.
fn sierpinski(n: u8) -> Box {
    if n == 0 {
        Box::from('*')
    } else {
        let s = sierpinski(n - 1);
        let base = Box::hconcat([s.clone(), Box::from(' '), s.clone()], Vertical::default());

        s.above(base, Horizontal::default())
    }
}

fn main() {
    print!("{}", sierpinski(5))
}
