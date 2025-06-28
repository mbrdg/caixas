use caixas::Box;

/// Creates a spiral with `n` turns.
fn spiral(n: u8) -> Box {
    if n == 0 {
        Box::from('+')
    } else {
        let s = spiral(n - 1);
        debug_assert_eq!(s.width(), usize::from(n - 1) * 4 + 1);
        debug_assert_eq!(s.height(), usize::from(n - 1) * 2 + 1);

        let (w, h) = (s.width(), s.height());
        let vbar = || Box::fill('|', 1, h);

        Box::grid([
            vec![Box::from("| +"), Box::fill('-', w, 1), Box::from('+')],
            vec![vbar(), Box::from(' '), s, Box::from(' '), vbar()],
            vec![Box::from('+'), Box::fill('-', w + 2, 1), Box::from('+')],
        ])
    }
}

fn main() {
    print!("{}", spiral(4))
}
