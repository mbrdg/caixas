use std::{cmp::max, fmt::Display};

use caixas::{Box, Horizontal, Vertical};

#[derive(Debug, Clone, Copy)]
enum Rating {
    Bad,
    Poor,
    Medium,
    Good,
    Excelent,
}

impl Display for Rating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rating::Bad => f.write_str("*"),
            Rating::Poor => f.write_str("**"),
            Rating::Medium => f.write_str("***"),
            Rating::Good => f.write_str("****"),
            Rating::Excelent => f.write_str("*****"),
        }
    }
}

#[derive(Clone)]
struct Book {
    title: String,
    author: String,
    rating: Rating,
    price: f32,
}

/// Creates a table (opinionated) formatted table with books' information.
fn make_table<'a, I>(books: I) -> Box
where
    I: Iterator<Item = &'a Book> + Clone,
{
    /// Helper function to create any necessary columns.
    fn make_column<'a>(
        books: impl Iterator<Item = &'a Book>,
        f: impl FnMut(&'a Book) -> String,
        alignment: Horizontal,
    ) -> Box {
        Box::vconcat(books.map(f).map(Box::from), alignment)
    }

    let columns = [
        (
            Box::from("Title"),
            make_column(books.clone(), |b| b.title.clone(), Horizontal::Left),
        ),
        (
            Box::from("Author"),
            make_column(books.clone(), |b| b.author.clone(), Horizontal::Left),
        ),
        (
            Box::from("Rating"),
            make_column(books.clone(), |b| b.rating.to_string(), Horizontal::Left),
        ),
        (
            Box::from("Price"),
            make_column(books, |b| format!("{:.2}", b.price), Horizontal::Right),
        ),
    ];

    let titled = columns.map(|(header, column)| {
        let hbar = Box::fill('-', max(header.width(), column.width()) + 2, 1);
        Box::vconcat([header, hbar, column], Horizontal::default())
    });

    let h = titled[0].height();
    let vbar = || Box::fill('|', 1, h);

    titled.into_iter().fold(vbar(), |acc, b| {
        Box::hconcat([acc, b, vbar()], Vertical::default())
    })
}

fn main() {
    let books = [
        Book {
            title: String::from("Waiting for Good Dough"),
            author: String::from("Samuel Biscuit"),
            rating: Rating::Good,
            price: 23.86,
        },
        Book {
            title: String::from("The Bun Also Rises"),
            author: String::from("Ernest Hemingwaffle"),
            rating: Rating::Excelent,
            price: 9.86,
        },
        Book {
            title: String::from("Yeast of Eden"),
            author: String::from("John Sconebeck"),
            rating: Rating::Poor,
            price: 6.00,
        },
        Book {
            title: String::from("One Hundred Years of Solid Food"),
            author: String::from("G. Gordita Marquez"),
            rating: Rating::Good,
            price: 17.00,
        },
        Book {
            title: String::from("The Theory of Yeast"),
            author: String::from("Mitchell Breadloaf"),
            rating: Rating::Bad,
            price: 15.55,
        },
        Book {
            title: String::from("The Encyclopedia of British Food"),
            author: String::from("Sam W. Lollypop"),
            rating: Rating::Medium,
            price: 33.33,
        },
    ];

    print!("{}", make_table(books.iter()))
}
