use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};
use std::path::PathBuf;

pub fn stdin_lines() -> impl Iterator<Item = io::Result<String>> {
    BufReader::new(io::stdin()).lines()
}

pub fn stdin_string() -> io::Result<String> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    Ok(buf)
}

pub fn file_lines(
    path: impl AsRef<PathBuf>,
) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let f = File::open(path.as_ref())?;
    Ok(BufReader::new(f).lines())
}

pub fn min_max<T: PartialOrd + Copy>(xs: &[T]) -> Option<(T, T)> {
    match xs {
        &[] => None,
        &[x] => Some((x, x)),
        &[x, y, ref xs @ ..] => {
            let (im, ix) = if x < y { (x, y) } else { (y, x) };
            if let Some((mn, mx)) = min_max(xs) {
                let min = if im < mn { im } else { mn };
                let max = if ix > mx { ix } else { mx };
                Some((min, max))
            } else {
                Some((im, ix))
            }
        }
    }
}

pub fn cartesian_product<'a, A: 'a + Clone, B: 'a, I1, I2>(
    it1: I1,
    it2: I2,
) -> impl Iterator<Item = (A, B)> + 'a
where
    I1: 'a + Iterator<Item = A>,
    I2: 'a + Iterator<Item = B> + Clone,
{
    it1.flat_map(move |i| it2.clone().map(move |j| (i.clone(), j)))
}
