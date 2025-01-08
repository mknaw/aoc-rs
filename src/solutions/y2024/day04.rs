#[inline(always)]
fn check_horizontal<const N: usize>(s: &[u8; N]) -> u32 {
    let mut count = 0;
    for i in 0..N - 3 {
        if s[i..i + 4] == *b"XMAS" {
            count += 1;
        } else if s[i..i + 4] == [b'S', b'A', b'M', b'X'] {
            count += 1;
        }
    }
    count
}

#[inline(always)]
fn check_vertical<const N: usize>(
    i: usize,
    curr: &[u8; N],
    p1: &[u8; N],
    p2: &[u8; N],
    p3: &[u8; N],
) -> bool {
    if curr[i] == b'X' && p1[i] == b'M' && p2[i] == b'A' && p3[i] == b'S' {
        true
    } else if curr[i] == b'S' && p1[i] == b'A' && p2[i] == b'M' && p3[i] == b'X' {
        true
    } else {
        false
    }
}

#[inline(always)]
fn check_diag_nw<const N: usize>(
    i: usize,
    curr: &[u8; N],
    p1: &[u8; N],
    p2: &[u8; N],
    p3: &[u8; N],
) -> bool {
    if curr[i] == b'X' && p1[i - 1] == b'M' && p2[i - 2] == b'A' && p3[i - 3] == b'S' {
        true
    } else if curr[i] == b'S' && p1[i - 1] == b'A' && p2[i - 2] == b'M' && p3[i - 3] == b'X' {
        true
    } else {
        false
    }
}

#[inline(always)]
fn check_diag_ne<const N: usize>(
    i: usize,
    curr: &[u8; N],
    p1: &[u8; N],
    p2: &[u8; N],
    p3: &[u8; N],
) -> bool {
    if curr[i] == b'X' && p1[i + 1] == b'M' && p2[i + 2] == b'A' && p3[i + 3] == b'S' {
        true
    } else if curr[i] == b'S' && p1[i + 1] == b'A' && p2[i + 2] == b'M' && p3[i + 3] == b'X' {
        true
    } else {
        false
    }
}

fn _solve_a<const N: usize>(s: &[u8]) -> u32 {
    let mut count = 0;

    let mut i = 0;
    let mut j = 0;

    let mut p3 = &[0u8; N];
    let mut p2 = &[0u8; N];
    let mut p1 = &[0u8; N];
    let mut curr: &[u8; N] = s[0..N].try_into().unwrap();

    for c in s {
        if *c == b'\n' {
            p1 = curr.try_into().unwrap();
            if j > 0 {
                p2 = s[(j - 1) * (N + 1)..(j - 1) * (N + 1) + N]
                    .try_into()
                    .unwrap();
            }
            if j > 1 {
                p3 = s[(j - 2) * (N + 1)..(j - 2) * (N + 1) + N]
                    .try_into()
                    .unwrap();
            }

            i = 0;
            j += 1;
            if j * (N + 1) + j < s.len() {
                curr = s[j * (N + 1)..j * (N + 1) + N].try_into().unwrap();
            } else {
                break;
            }

            continue;
        }

        if i == 0 {
            count += check_horizontal(curr);
        }
        if j > 2 {
            if check_vertical(i, curr, p1, p2, p3) {
                count += 1;
            }
            if i > 2 && check_diag_nw(i, curr, p1, p2, p3) {
                count += 1;
            }
            if i < N - 3 && check_diag_ne(i, curr, p1, p2, p3) {
                count += 1;
            }
        }

        i += 1;
    }

    count
}

pub fn solve_a(s: &[u8]) -> u32 {
    _solve_a::<140>(s)
}

#[inline(always)]
fn check_x<const N: usize>(i: usize, curr: &[u8; N], p1: &[u8; N], p2: &[u8; N]) -> bool {
    if p1[i - 1] != b'A' {
        return false;
    }
    if ((curr[i] == b'S' && p2[i - 2] == b'M') || (curr[i] == b'M' && p2[i - 2] == b'S'))
        && ((curr[i - 2] == b'S' && p2[i] == b'M') || (curr[i - 2] == b'M' && p2[i] == b'S'))
    {
        true
    } else {
        false
    }
}

pub fn _solve_b<const N: usize>(s: &[u8]) -> u32 {
    let mut count = 0;

    let mut i = 0;
    let mut j = 0;

    let mut p2 = &[0u8; N];
    let mut p1 = &[0u8; N];
    let mut curr: &[u8; N] = s[0..N].try_into().unwrap();

    for c in s {
        if *c == b'\n' {
            p1 = curr.try_into().unwrap();
            if j > 0 {
                p2 = s[(j - 1) * (N + 1)..(j - 1) * (N + 1) + N]
                    .try_into()
                    .unwrap();
            }

            i = 0;
            j += 1;
            if j * (N + 1) + j < s.len() {
                curr = s[j * (N + 1)..j * (N + 1) + N].try_into().unwrap();
            } else {
                break;
            }

            continue;
        }

        if j > 1 && i > 1 && check_x(i, curr, p1, p2) {
            count += 1;
        }

        i += 1;
    }

    count
}

pub fn solve_b(s: &[u8]) -> u32 {
    _solve_b::<140>(s)
}

#[test]
fn test_solve_a() {
    let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    assert_eq!(_solve_a::<10>(input.as_bytes()), 18);
}

#[test]
fn test_solve_b() {
    let input = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    assert_eq!(_solve_b::<10>(input.as_bytes()), 9);
}
