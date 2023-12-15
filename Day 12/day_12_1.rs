use std::fs;

const PARSE_ERROR: &str = "Parsing issue";
const FILE_ERROR: &str = "File Operation Error";

fn ways(i: usize, j: usize, dp: &mut Vec<Vec<i32>>, spring: &str, groups: &Vec<usize>) -> i32 {
    if j >= groups.len() {
        let mut consistent: bool = true;
        for x in i..spring.len() {
            consistent &= (spring.chars().nth(x).unwrap() != '#');
        }
        return consistent as i32;
    }

    if i >= spring.len() {
        return 0;
    }

    if dp[i][j] != -1 {
        return dp[i][j];
    }

    assert!(i < spring.len());
    assert!(j < groups.len());
    let mut ans: i32 = 0;
    if spring.chars().nth(i).unwrap() == '?' || spring.chars().nth(i).unwrap() == '.' {
        ans += ways(i + 1, j, dp, &spring, &groups);
    }
    if (spring.chars().nth(i).unwrap() == '?' || spring.chars().nth(i).unwrap() == '#')
        && (groups[j] + i <= spring.len())
    {
        let mut consistent: bool = true;
        assert!(i + groups[j] <= spring.len());
        for x in i..(i + groups[j]) {
            assert!(x < spring.len());
            consistent &=
                (spring.chars().nth(x).unwrap() == '?' || spring.chars().nth(x).unwrap() == '#');
        }
        if i + groups[j] < spring.len() {
            consistent &= spring.chars().nth(i + groups[j]).unwrap() != '#';
        }
        if consistent {
            ans += ways(i + groups[j] + 1, j + 1, dp, spring, groups);
        }
    }
    dp[i][j] = ans;
    return ans;
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect(FILE_ERROR);
    let springs: Vec<&str> = input.lines().collect();
    let mut ans: i32 = 0;
    for spring in springs {
        let data: Vec<&str> = spring.split_ascii_whitespace().collect();
        let groups: Vec<usize> = data[1]
            .split(',')
            .into_iter()
            .map(|x| x.parse::<usize>().expect(PARSE_ERROR))
            .collect();
        let mut dp: Vec<Vec<i32>> = Vec::with_capacity(data[0].len() + 1);
        dp.resize(data[0].len(), Vec::with_capacity(groups.len() + 1));
        for x in 0..dp.len() {
            dp[x].resize(groups.len() + 1, -1);
        }
        let cur_ways = ways(0, 0, &mut dp, data[0], &groups);
        ans += cur_ways;
    }

    println!("{}", ans);
}

/*
?#?#?#?#?#?#?#?
.#.#?#?#?#?#?#?
*/
