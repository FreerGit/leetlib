pub fn flood_fill(mut image: Vec<Vec<i32>>, sr: i32, sc: i32, color: i32) -> Vec<Vec<i32>> {
    let (sr, sc) = (sr as usize, sc as usize);
    let start_color = image[sr as usize][sc as usize];
    if color != start_color {
        dfs(&mut image, sr, sc, color, start_color);
    }
    image
}

pub fn dfs(image: &mut Vec<Vec<i32>>, sr: usize, sc: usize, color: i32, start_color: i32) -> () {
    if image[sr][sc] == start_color {
        image[sr][sc] = color;
        if sr > 0 {
            dfs(image, sr - 1, sc, color, start_color);
        }
        if sr + 1 < image.len() {
            dfs(image, sr + 1, sc, color, start_color);
        }
        if sc > 0 {
            dfs(image, sr, sc - 1, color, start_color);
        }
        if sc + 1 < image[0].len() {
            dfs(image, sr, sc + 1, color, start_color);
        }
    }
}

#[cfg(test)]
mod test {
    use crate::lc_0733_flood_fill::flood_fill;

    #[test]
    pub fn ex1() {
        let starting_colors: &mut Vec<Vec<i32>> =
            &mut vec![[1, 1, 1].to_vec(), [1, 1, 0].to_vec(), [1, 0, 1].to_vec()];
        let output = vec![[2, 2, 2], [2, 2, 0], [2, 0, 1]];
        assert_eq!(flood_fill(starting_colors.to_vec(), 1, 1, 2), output);
    }
}
