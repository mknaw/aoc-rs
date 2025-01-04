use std::collections::HashMap;

struct Parser<'a> {
    data: &'a [u8],
    idx: usize,
}

impl<'a> Parser<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, idx: 0 }
    }
}

impl Iterator for Parser<'_> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let mut num = None;
        while self.idx < self.data.len() {
            match self.data[self.idx] {
                b' ' | b'\n' => {
                    self.idx += 1;
                    if num.is_some() {
                        return num;
                    }
                }
                d => {
                    num = num.map_or_else(
                        || Some((d - b'0') as u32),
                        |x| Some(10 * x + (d - b'0') as u32),
                    );
                    self.idx += 1;
                }
            }
        }
        num
    }
}

pub fn solve_a(input: &[u8]) -> u32 {
    let mut parser = Parser::new(input);
    let mut lefts = Vec::<u32>::with_capacity(1000);
    let mut rights = Vec::<u32>::with_capacity(1000);

    loop {
        let l = parser.next();
        let r = parser.next();
        if l.is_none() || r.is_none() {
            break;
        }
        lefts.push(l.unwrap());
        rights.push(r.unwrap());
    }

    lefts.sort();
    rights.sort();

    lefts
        .iter()
        .zip(rights.iter())
        .fold(0, |acc, (&l, &r)| acc + l.abs_diff(r))
}

pub fn solve_b(input: &[u8]) -> u32 {
    let mut parser = Parser::new(input);
    let mut lefts = Vec::<u32>::with_capacity(1000);
    let mut counts = HashMap::<u32, u16>::with_capacity(1000);

    loop {
        let l = parser.next();
        let r = parser.next();
        if l.is_none() || r.is_none() {
            break;
        }
        lefts.push(l.unwrap());
        if let Some(x) = counts.get_mut(&r.unwrap()) {
            *x += 1;
        } else {
            counts.insert(r.unwrap(), 1);
        }
    }

    lefts.iter().fold(0, |acc, &x| {
        if let Some(count) = counts.get(&x) {
            acc + (*count as u32) * x
        } else {
            acc
        }
    })
}

#[test]
fn test_solve_a() {
    let input = r"
3   4
4   3
2   5
1   3
3   9
3   3
        ";
    assert_eq!(solve_a(input.as_bytes()), 11);
}

#[test]
fn test_solve_b() {
    let input = r"
3   4
4   3
2   5
1   3
3   9
3   3
        ";
    assert_eq!(solve_b(input.as_bytes()), 31);
}
