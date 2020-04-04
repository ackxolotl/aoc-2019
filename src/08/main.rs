#![allow(clippy::naive_bytecount)]

use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;

struct SpaceImage<'a, T: ?Sized, X> {
    data: &'a T,
    width: usize,
    height: usize,
    _phantom: PhantomData<X>,
}

impl<'a, T: ?Sized, X> SpaceImage<'a, T, X>
where
    T: AsRef<[X]>,
{
    pub fn new(data: &'a T, width: usize, height: usize) -> SpaceImage<'a, T, X> {
        assert_eq!(data.as_ref().len() % width * height, 0);

        SpaceImage {
            data,
            width,
            height,
            _phantom: PhantomData,
        }
    }

    pub fn get_layer(&self, layer: usize) -> Option<&'a [X]> {
        if layer >= self.layers() {
            None
        } else {
            let from = (self.width * self.height) * layer;
            let to = (self.width * self.height) + from;

            Some(&self.data.as_ref()[from..to])
        }
    }

    fn layers(&self) -> usize {
        self.data.as_ref().len() / (self.width * self.height)
    }
}

struct SpaceIterator<'a, T: ?Sized, X> {
    s: &'a SpaceImage<'a, T, X>,
    i: usize,
}

impl<'a, T: ?Sized, X> Iterator for SpaceIterator<'a, T, X>
where
    T: AsRef<[X]>,
{
    type Item = &'a [X];

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.s.layers() {
            None
        } else {
            self.i += 1;
            Some(self.s.get_layer(self.i - 1).unwrap())
        }
    }
}

impl<'a, T: ?Sized, X> IntoIterator for &'a SpaceImage<'a, T, X>
where
    T: AsRef<[X]>,
{
    type Item = &'a [X];
    type IntoIter = SpaceIterator<'a, T, X>;

    fn into_iter(self) -> Self::IntoIter {
        SpaceIterator { s: self, i: 0 }
    }
}

fn main() {
    let mut input = String::new();

    File::open("./src/08/input.txt")
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let data: Vec<u8> = input
        .chars()
        .map(|x| x.to_digit(10).unwrap() as u8)
        .collect();

    let s = SpaceImage::new(&data, 25, 6);

    let l = s
        .into_iter()
        .min_by_key(|x| x.iter().filter(|y| **y == 0).count())
        .unwrap();

    let ones = l.iter().filter(|x| **x == 1).count();
    let twos = l.iter().filter(|x| **x == 2).count();

    println!("'1's times '2's: {}\n", ones * twos);

    println!("Final image:");

    for i in 0..(25 * 6) {
        if i % 25 == 0 {
            println!();
        }

        match s.into_iter().map(|x| x[i]).find(|x| *x != 2) {
            Some(0) => print!("\x1B[30m█"),
            Some(1) => print!("\x1B[0m█"),
            _ => panic!("wrong color"),
        }
    }
}
