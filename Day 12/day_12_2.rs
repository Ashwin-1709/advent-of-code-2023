use std::fs;
use std::time::Instant;

const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

fn ways(
    i: usize,
    j: usize,
    dp: &mut Vec<Vec<i64>>,
    spring: &Vec<char>,
    groups: &Vec<usize>,
) -> i64 {
    if j >= groups.len() {
        let mut consistent: bool = true;
        for x in i..spring.len() {
            consistent &= (spring[x] != '#');
        }
        return consistent as i64;
    }

    if i >= spring.len() {
        return 0;
    }

    if dp[i][j] != -1 {
        return dp[i][j];
    }

    assert!(i < spring.len());
    assert!(j < groups.len());
    let mut ans: i64 = 0;
    if spring[i] == '?' || spring[i] == '.' {
        ans += ways(i + 1, j, dp, &spring, &groups);
    }
    if (spring[i] == '?' || spring[i] == '#') && (groups[j] + i <= spring.len()) {
        let mut consistent: bool = true;
        assert!(i + groups[j] <= spring.len());
        for x in i..(i + groups[j]) {
            assert!(x < spring.len());
            consistent &= (spring[x] == '?' || spring[x] == '#');
        }
        if i + groups[j] < spring.len() {
            consistent &= spring[i + groups[j]] != '#';
        }
        if consistent {
            ans += ways(i + groups[j] + 1, j + 1, dp, spring, groups);
        }
    }
    dp[i][j] = ans;
    return ans;
}

fn main() {
    let start = Instant::now();
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let springs: Vec<&str> = input.lines().collect();
    let mut ans: i64 = 0;
    for spring in springs {
        let data: Vec<&str> = spring.split_ascii_whitespace().collect();
        let groups: Vec<usize> = data[1]
            .split(',')
            .into_iter()
            .map(|x| x.parse::<usize>().expect(PARSE_ERROR))
            .collect();
        let fold: Vec<usize> = groups.repeat(5);
        let transformed_string: String = std::iter::repeat(data[0])
            .take(5)
            .collect::<Vec<&str>>()
            .join("?");

        let pass_data: &str = &transformed_string;
        let mut dp: Vec<Vec<i64>> = Vec::with_capacity(pass_data.len() + 1);
        dp.resize(pass_data.len(), Vec::with_capacity(fold.len() + 1));
        for x in 0..dp.len() {
            dp[x].resize(fold.len() + 1, -1);
        }
        let str_mod: Vec<char> = pass_data.chars().collect();
        let cur_ways = ways(0, 0, &mut dp, &str_mod, &fold);
        ans += cur_ways;
    }

    println!("{}", ans);
    let elapsed = start.elapsed();
    println!("Time taken: {:?}", elapsed);
}
