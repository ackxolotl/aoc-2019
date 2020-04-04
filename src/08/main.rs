#![allow(clippy::naive_bytecount)]

use std::fs::File;
use std::io::Read;
use std::marker::PhantomData;

struct SpaceImage<'a, T: 'a, X: 'a> {
    data: &'a T,
    width: usize,
    height: usize,
    _phantom: PhantomData<X>,
}

impl<'a, T, X> SpaceImage<'a, T, X>
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

    pub fn layers(&self) -> usize {
        self.data.as_ref().len() / (self.width * self.height)
    }

    pub fn get_layer(&self, layer: usize) -> Option<&[X]> {
        if layer >= self.layers() {
            None
        } else {
            let from = (self.width * self.height) * layer;
            let to = (self.width * self.height) + from;

            Some(&self.data.as_ref()[from..to])
        }
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

    let mut zeros = (0, std::usize::MAX);

    for i in 0..s.layers() {
        let z = s.get_layer(i).unwrap().iter().filter(|x| **x == 0).count();

        if z < zeros.1 {
            zeros = (i, z);
        }
    }

    let l = s.get_layer(zeros.0).unwrap();

    let ones = l.iter().filter(|x| **x == 1).count();
    let twos = l.iter().filter(|x| **x == 2).count();

    println!("1's times 2's: {}\n", ones * twos);

    let mut final_image = vec![0; 25 * 6];

    for (i, fin) in final_image.iter_mut().enumerate() {
        for j in 0..s.layers() {
            *fin = s.get_layer(j).unwrap()[i];

            if *fin != 2 {
                break;
            }
        }
    }

    println!("Final image: \n");

    for h in 0..6 {
        for w in 0..25 {
            match final_image[h * 25 + w] {
                0 => print!("\x1B[30m█"),
                1 => print!("\x1B[0m█"),
                _ => panic!("no color"),
            }
        }

        println!();
    }
}
