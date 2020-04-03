pub struct Permutation {
    current: [u8; 5],
    state: [u8; 5],
    q: usize,
}

impl Permutation {
    pub fn new(initial: [u8; 5]) -> Permutation {
        Permutation {
            current: initial,
            state: [0, 0, 0, 0, 0],
            q: 0,
        }
    }
}

impl Iterator for Permutation {
    type Item = [u8; 5];

    fn next(&mut self) -> Option<[u8; 5]> {
        if self.q == 0 {
            self.q += 1;
            return Some(self.current);
        } else if self.q >= 120 {
            return None;
        } else {
            let mut i = 0;

            while i < 5 {
                if self.state[i] < i as u8 {
                    let swap = if i % 2 == 0 {
                        0
                    } else {
                        self.state[i] as usize
                    };

                    self.current.swap(swap, i);

                    self.state[i] += 1;
                    self.q += 1;

                    return Some(self.current);
                } else {
                    self.state[i] = 0;
                    i += 1;
                }
            }

            unreachable!()
        }
    }
}
