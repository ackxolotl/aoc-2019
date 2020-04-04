use std::marker::PhantomData;

const MAX_SIZE: usize = 16;

pub struct Permutation<'a, T: ?Sized, X> {
    current: &'a mut T,
    state: [u8; MAX_SIZE],
    q: usize,
    _phantom: PhantomData<X>,
}

impl<'a, T, X> Permutation<'a, T, X>
where
    T: ToOwned + AsMut<[X]>,
{
    pub fn new(data: &'a mut T) -> Permutation<'a, T, X> {
        assert!(
            data.as_mut().len() <= MAX_SIZE,
            "too many elements to permute, max = {}",
            MAX_SIZE
        );

        Permutation {
            current: data,
            state: [0; MAX_SIZE],
            q: 0,
            _phantom: PhantomData,
        }
    }
}

impl<'a, T, X> Iterator for Permutation<'a, T, X>
where
    T: ToOwned + AsMut<[X]>,
{
    type Item = T::Owned;

    fn next(&mut self) -> Option<Self::Item> {
        if self.q == 0 {
            self.q += 1;
            Some(self.current.to_owned())
        } else if self.q >= 120 {
            None
        } else {
            let mut i = 0;

            while i < 5 {
                if self.state[i] < i as u8 {
                    let swap = if i % 2 == 0 {
                        0
                    } else {
                        self.state[i] as usize
                    };

                    self.current.as_mut().swap(swap, i);

                    self.state[i] += 1;
                    self.q += 1;

                    return Some(self.current.to_owned());
                } else {
                    self.state[i] = 0;
                    i += 1;
                }
            }

            unreachable!()
        }
    }
}
