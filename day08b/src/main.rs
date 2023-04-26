fn main() {
    let s = include_str!("../input.txt");

    let mat = s.lines().map(|s| s.as_bytes()).collect::<Vec<&[u8]>>();
    let m = mat.len();
    let n = mat[0].len();

    let mut views = vec![vec![1usize; n]; m];

    for r in 1..(m - 1) {
        for c in 1..(n - 1) {
            views[r][c] *= (r + 1..m - 1)
                .take_while(|i| mat[*i][c] < mat[r][c])
                .count()
                + 1;

            views[r][c] *= (1..r).rev().take_while(|i| mat[*i][c] < mat[r][c]).count() + 1;

            views[r][c] *= (c + 1..n - 1)
                .take_while(|i| mat[r][*i] < mat[r][c])
                .count()
                + 1;

            views[r][c] *= (1..c).rev().take_while(|i| mat[r][*i] < mat[r][c]).count() + 1;
        }
    }

    let ans = views
        .into_iter()
        .map(|v| v.into_iter().max().unwrap())
        .max()
        .unwrap();

    println!("{}", ans);
}
