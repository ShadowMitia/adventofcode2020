use std::collections::{HashMap, HashSet};

fn find_sum_pair_in_list(input: &[u32], value: u32) -> (u32, u32) {
    for a in input {
        for b in input {
            if a != b && a + b == value {
                return (*a, *b);
            }
        }
    }
    (0, 0)
}

fn find_sum_triple_in_list(input: &[u32], value: u32) -> (u32, u32, u32) {
    for a in input {
        for b in input {
            for c in input {
                if a != b && b != c && a + b + c == value {
                    return (*a, *b, *c);
                }
            }
        }
    }
    (0, 0, 0)
}

fn is_valid_password(password: &str, policy: &str) -> bool {
    let res = policy.split(' ').collect::<Vec<&str>>();
    let character = res[1];
    let res = res[0].split('-').collect::<Vec<&str>>();
    let min = res[0].parse().unwrap();
    let max = res[1].parse().unwrap();

    let mut count = 0;

    for c in password.chars() {
        if c.to_string() == character {
            count += 1;
        }
    }

    count <= max && count >= min
}

fn is_valid_password2(password: &str, policy: &str) -> bool {
    let res = policy.split(' ').collect::<Vec<&str>>();
    let character = res[1];
    let res = res[0].split('-').collect::<Vec<&str>>();
    let min = res[0].parse().unwrap();
    let max = res[1].parse().unwrap();

    let a = password.get(min - 1..min).unwrap();
    let b = password.get(max - 1..max).unwrap();

    (a == character || b == character) && a != b
}

fn count_trees_encountered((x, y): (usize, usize), input: &str) -> u32 {
    let mut position: (usize, usize) = (x, y);

    let map = input.lines().collect::<Vec<&str>>();
    let width = map[0].len();
    let height = map.len();

    let mut counter = 0;
    while position.1 < height {
        let x_prime = position.0.rem_euclid(width);
        if &map[position.1][x_prime..=x_prime] == "#" {
            counter += 1;
        }
        position.0 += x;
        position.1 += y;
    }

    counter
}

fn is_valid_passport(passport: &str) -> bool {
    let required_fields = vec![
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", /* "cid" */
    ];

    for field in required_fields {
        if passport.find(field).is_none() {
            return false;
        }
    }

    true
}

fn byr_test(val: &str) -> bool {
    if val.len() != 4 {
        return false;
    }
    match val.parse::<u32>() {
        Ok(val) => val >= 1920 && val <= 2002,
        Err(_) => false,
    }
}

fn iyr_test(val: &str) -> bool {
    if val.len() != 4 {
        return false;
    }
    match val.parse::<u32>() {
        Ok(val) => val >= 2010 && val <= 2020,
        Err(_) => false,
    }
}

fn eyr_test(val: &str) -> bool {
    if val.len() != 4 {
        return false;
    }
    match val.parse::<u32>() {
        Ok(val) => val >= 2020 && val <= 2030,
        Err(_) => false,
    }
}

fn hgt_test(val: &str) -> bool {
    match val[..val.len() - 2].parse::<u32>() {
        Ok(res) => {
            if val.ends_with("cm") {
                return res >= 150 && res <= 193;
            }

            if val.ends_with("in") {
                return res >= 59 && res <= 76;
            }
            false
        }
        Err(_) => false,
    }
}

fn hcl_test(val: &str) -> bool {
    if val.len() != 7 {
        return false;
    }
    val[1..]
        .chars()
        .all(|c| c.is_numeric() || c >= 'a' && c <= 'f')
}

fn ecl_test(val: &str) -> bool {
    let eyes = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

    for eye in eyes {
        if val.find(eye).is_some() {
            return true;
        }
    }
    false
}

fn pid_test(val: &str) -> bool {
    val.len() == 9 && val.chars().all(|c| c.is_numeric())
}

type FieldWithValidationFunc<'a> = (&'a str, for<'r> fn(&'r str) -> bool);

fn is_valid_passport_with_data_validation(passport: &str) -> bool {
    let required_fields: Vec<FieldWithValidationFunc> = vec![
        ("byr", byr_test),
        ("iyr", iyr_test),
        ("eyr", eyr_test),
        ("hgt", hgt_test),
        ("hcl", hcl_test),
        ("ecl", ecl_test),
        ("pid", pid_test),
        /* "cid" */
    ];

    for (field, data_validator) in required_fields {
        let index = passport.find(field);
        if index.is_none() {
            return false;
        }
        let index = index.unwrap();
        let next_index = passport[index + 4..].split(' ').next().unwrap();
        if !data_validator(next_index) {
            return false;
        }
    }
    true
}

fn find_airplane_seat(input: &str) -> (u32, u32, u32) {
    let mut row_data = Vec::new();
    for el in 0..=127 {
        row_data.push(el);
    }

    let mut column_data = Vec::new();
    for el in 0..=7 {
        column_data.push(el);
    }

    let mut column = column_data.as_slice();

    let mut row = row_data.as_slice();

    for c in input.chars() {
        match c {
            'F' => row = &row[..row.len() / 2],
            'B' => row = &row[row.len() / 2..],
            'L' => column = &column[..column.len() / 2],
            'R' => column = &column[column.len() / 2..],
            _ => panic!("Can't happen"),
        }
    }

    let id = row[0] * 8 + column[0];

    (row[0], column[0], id)
}

fn get_sums(numbers: &[u64]) -> Vec<u64> {
    let mut res = Vec::new();
    for n1 in numbers {
        for n2 in numbers {
            if n1 != n2 {
                res.push(n1 + n2);
            }
        }
    }
    res
}

fn find_first_invalid_xmas_number(numbers: &[u64], preamble_size: u64) -> Option<u64> {
    for k in (preamble_size as usize + 1)..numbers.len() {
        let sums = get_sums(&numbers[(k - preamble_size as usize - 1)..k]);

        if sums.iter().find(|&&val| val == numbers[k]).is_none() {
            return Some(numbers[k]);
        }
    }

    None
}

fn find_contiguous_set_sum_to(number: u64, numbers: &[u64]) -> Vec<u64> {
    let mut i = 0;
    let mut j = 2;
    loop {
        let sum = numbers.iter().skip(i).take(j).sum::<u64>();
        if sum == number {
            return numbers
                .iter()
                .skip(i)
                .take(j)
                .copied()
                .collect::<Vec<u64>>();
        }

        if sum > number {
            i = 0;
            j += 1;
        }
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn day1_part1_test() {
        let expense_report = vec![1721, 979, 366, 299, 675, 14456];
        let (a, b) = find_sum_pair_in_list(&expense_report, 2020);
        assert_eq!(a * b, 514579);
    }

    #[test]
    fn day1_part2_test() {
        let expense_report = vec![1721, 979, 366, 299, 675, 14456];
        let (a, b, c) = find_sum_triple_in_list(&expense_report, 2020);
        assert_eq!(a * b * c, 241861950);
    }

    #[test]
    fn day2_part1_test() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

        let mut policies = Vec::new();
        let mut passwords = Vec::new();

        for line in input.lines() {
            let res = line.split(':').collect::<Vec<&str>>();
            policies.push(res[0]);
            passwords.push(res[1].trim());
        }

        let mut count = 0;

        for (&password, policy) in passwords.iter().zip(policies) {
            if is_valid_password(password, policy) {
                count += 1;
            }
        }

        assert_eq!(count, 2);
    }

    #[test]
    fn day2_part2_test() {
        let input = "1-3 a: abcde\n1-3 b: cdefg\n2-9 c: ccccccccc";

        let mut policies = Vec::new();
        let mut passwords = Vec::new();

        for line in input.lines() {
            let res = line.split(':').collect::<Vec<&str>>();
            policies.push(res[0]);
            passwords.push(res[1].trim());
        }

        let mut count = 0;

        for (&password, policy) in passwords.iter().zip(policies) {
            if is_valid_password2(password, policy) {
                count += 1;
            }
        }

        assert_eq!(count, 1);
    }

    #[test]
    fn day3_part1_test() {
        let input = r#"..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#"#;

        let trees = count_trees_encountered((3, 1), input);

        assert_eq!(trees, 7);
    }

    #[test]
    fn day4_part1_test() {
        let input = r#"ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm
iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in"#;

        let mut passports = Vec::new();
        let mut passport = String::new();

        for line in input.lines() {
            if line.is_empty() {
                passports.push(passport);
                passport = String::new();
                continue;
            }
            passport.push_str(line);
        }

        passports.push(passport);

        let mut counter = 0;

        for passport in passports {
            if is_valid_passport(&passport) {
                counter += 1;
            }
        }

        assert_eq!(counter, 2);
    }

    #[test]
    fn day4_part2_test() {
        let invalid_input = r#"eyr:1972 cid:100
        hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926
        
        iyr:2019
        hcl:#602927 eyr:1967 hgt:170cm
        ecl:grn pid:012533040 byr:1946
        
        hcl:dab227 iyr:2012
        ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277
        
        hgt:59cm ecl:zzz
        eyr:2038 hcl:74454a iyr:2023
        pid:3556412378 byr:2007"#;

        let valid_input = r#"
        pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        "#;

        assert_eq!(byr_test("2002"), true);
        assert_eq!(byr_test("2003"), false);

        assert_eq!(hgt_test("60in"), true);
        assert_eq!(hgt_test("190cm"), true);
        assert_eq!(hgt_test("190in"), false);
        assert_eq!(hgt_test("190"), false);

        assert_eq!(hcl_test("#123abc"), true);
        assert_eq!(hcl_test("#123abz"), false);
        assert_eq!(hcl_test("123abc"), false);

        assert_eq!(ecl_test("brn"), true);
        assert_eq!(ecl_test("wat"), false);

        assert_eq!(pid_test("000000001"), true);
        assert_eq!(pid_test("0123456789"), false);

        let mut passports = Vec::new();
        let mut passport = String::new();

        for line in invalid_input.lines() {
            let line = line.trim();

            if line.is_empty() {
                passports.push(passport);
                passport = String::new();
                continue;
            }
            passport.push_str(line);
            passport.push(' ');
        }

        passports.push(passport);

        let mut counter = 0;

        for passport in passports {
            if is_valid_passport_with_data_validation(&passport) {
                counter += 1;
            }
        }

        assert_eq!(counter, 0);

        let mut passports = Vec::new();
        let mut passport = String::new();

        for line in valid_input.lines() {
            let line = line.trim();

            if line.is_empty() {
                passports.push(passport);
                passport = String::new();
                continue;
            }
            passport.push_str(line);
            passport.push(' ');
        }

        passports.push(passport);

        let mut counter = 0;

        for passport in passports {
            if is_valid_passport_with_data_validation(&passport) {
                counter += 1;
            }
        }

        assert_eq!(counter, 4);
    }

    #[test]
    fn day5_part1_test() {
        let input = "BFFFBBFRRR\nFFFBBBFRRR\nBBFFBBFRLL"
            .split('\n')
            .collect::<Vec<&str>>();
        let results = vec![(70, 7, 567), (14, 7, 119), (102, 4, 820)];

        for (i, r) in input.iter().zip(results) {
            assert_eq!(find_airplane_seat(i), r);
        }
    }

    #[test]
    fn day6_part1_test() {
        let input = r#"abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b"#
        .lines()
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

        let mut groups = Vec::new();
        let mut group = HashSet::new();

        for line in input {
            if line.is_empty() {
                groups.push(group);
                group = HashSet::new();
            }
            line.chars().for_each(|c| {
                group.insert(c);
            });
        }
        groups.push(group);

        assert_eq!(groups.iter().map(|g| g.len()).sum::<usize>(), 11);
    }

    #[test]
    fn day6_part2_test() {
        let input = r#"abc

        a
        b
        c
        
        ab
        ac
        
        a
        a
        a
        a
        
        b"#
        .lines()
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

        let mut groups = Vec::new();
        let mut group = HashMap::new();

        let mut group_size = 0;

        for line in input {
            if line.is_empty() {
                groups.push((group, group_size));
                group = HashMap::new();
                group_size = 0;
                continue;
            }
            line.chars().for_each(|c| {
                *group.entry(c).or_insert(0) += 1;
            });
            group_size += 1;
        }
        groups.push((group, group_size));

        println!("{:#?}", groups);

        assert_eq!(
            groups
                .iter()
                .map(|(group, size)| group
                    .iter()
                    .filter(|(_, i)| *i == size)
                    .map(|(c, _)| c)
                    .collect::<Vec<&char>>())
                .map(|m| m.len())
                .sum::<usize>(),
            6
        );
    }

    #[test]
    fn day7_part1_test() {
        // let input = r#"light red bags contain 1 bright white bag, 2 muted yellow bags.
        // dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        // bright white bags contain 1 shiny gold bag.
        // muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        // shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        // dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        // vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        // faded blue bags contain no other bags.
        // dotted black bags contain no other bags."#
        //     .lines()
        //     .map(|s| s.trim())
        //     .collect::<Vec<&str>>();

        // let mut rules = HashMap::new();

        // for line in input {
        //     let bag = &line[..line.find("bags").unwrap() - 1];
        //     let contained_bags = &line[line.find("contain").unwrap() + 8..];
        //     let contained_bags = contained_bags.split(',');
        //     let contained_bags = contained_bags.filter(|&s| s.find("no").is_none());
        //     let contained_bags = contained_bags
        //         .map(|data| {
        //             // println!("{:#?}", data.trim().split(' '));

        //             let data = data.trim().split(' ').collect::<Vec<&str>>();

        //             let quantity = data[0];
        //             let color = format!("{} {}", data[1], data[2]);

        //             (color, quantity.parse().unwrap())
        //         })
        //         .collect::<Vec<(String, u32)>>();

        //     // println!("{:#?}", contained_bags);

        //     rules.insert(bag, contained_bags);
        // }

        // println!("{:#?}", rules);

        // let bags = rules
        //     .iter()
        //     .scan(0, |state, (bag, inner_bags)| {

        //         if inner_bags.iter().any(|(s, _)| s == "shiny gold") {
        //             *state += 1;
        //         }

        //         for (bag, _) in inner_bags {
        //             if bag == "shiny gold" {
        //                 continue;
        //             }
        //             if state.iter().find(|&&s| s == *bag).is_none() {
        //                 for (inner_bag, _) in rules.get(bag.as_str()).unwrap().iter() {
        //                     if inner_bag == "shiny gold" {
        //                         *state += 1;
        //                     }
        //                 }
        //             }
        //         }

        //         // let res = inner_bags
        //         //     .iter()
        //         //     .filter(|(s, _)| s == "shiny gold")
        //         //     .filter(|(bag, _)| rules.get(bag.as_str()).unwrap().iter().count() > 0)
        //         //     .count();

        //         // println!("{}", res);

        //         Some(*state)
        //     }).last().unwrap();

        // println!("{:?}", bags);

        // assert_eq!(bags.len(), 4);
    }

    #[test]
    fn day8_part1_test() {
        let input = r#"nop +0
        acc +1
        jmp +4
        acc +3
        jmp -3
        acc -99
        acc +1
        jmp -4
        acc +6"#;

        let instructions = input.lines().map(|s| s.trim()).collect::<Vec<&str>>();
        let mut cpu = HGCCpu::new();
        let mut stackpointer_history = HashSet::new();
        let mut last_accumulator = 0;
        loop {
            cpu.execute(instructions[cpu.stackpointer as usize]);
            let res = stackpointer_history.insert(cpu.stackpointer);
            if !res {
                assert_eq!(last_accumulator, 5);
                return;
            }
            last_accumulator = cpu.accumulator;
        }
    }

    #[test]
    fn day9_part1_test() {
        let input = r#"35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576"#;

        let numbers = input
            .lines()
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let invalid = find_first_invalid_xmas_number(&numbers, 5);

        assert_eq!(invalid.unwrap(), 127);
    }

    #[test]
    fn day9_part2_test() {
        let input = r#"35
        20
        15
        25
        47
        40
        62
        55
        65
        95
        102
        117
        150
        182
        127
        219
        299
        277
        309
        576"#;

        let numbers = input
            .lines()
            .map(|s| s.trim().parse::<u64>().unwrap())
            .collect::<Vec<u64>>();

        let number = find_first_invalid_xmas_number(&numbers, 5).unwrap();

        let set = find_contiguous_set_sum_to(number, &numbers);
        let smallest = set.iter().min().unwrap();
        let highest = set.iter().max().unwrap();

        assert_eq!(smallest + highest, 62);
    }
}

struct HGCCpu {
    accumulator: isize,
    stackpointer: isize,
}

impl HGCCpu {
    fn new() -> Self {
        HGCCpu {
            accumulator: 0,
            stackpointer: 0,
        }
    }
    fn execute(&mut self, command: &str) {
        let data = command.split(' ').collect::<Vec<&str>>();
        // println!("{:?}", data);
        let command = data[0];
        let arg = data[1].trim().parse::<isize>().unwrap();
        // println!("Running {} {}", command, arg);
        match command {
            "nop" => self.stackpointer += 1,
            "acc" => {
                self.accumulator += arg;
                self.stackpointer += 1;
            }
            "jmp" => self.stackpointer += arg,
            _ => panic!("Unknown command {}", command),
        }
        // println!("cpu {} {}", self.stackpointer, self.accumulator);
    }
}

fn main() {
    // Day 1
    let expense_report = vec![
        1630, 1801, 1917, 1958, 1953, 1521, 1990, 1959, 1543, 1798, 638, 1499, 1977, 1433, 1532,
        1780, 1559, 1866, 1962, 1999, 1623, 1772, 1730, 1670, 1791, 1947, 1961, 1523, 959, 1998,
        1693, 1490, 1712, 910, 1635, 1837, 586, 1590, 1741, 1739, 1660, 1883, 1777, 1734, 1413,
        1456, 1511, 1957, 1738, 1685, 1677, 1419, 1566, 1639, 1578, 1922, 1856, 1946, 1965, 1649,
        1854, 1610, 1806, 1424, 1616, 218, 1678, 1992, 1985, 903, 1626, 1412, 1964, 671, 1692,
        1571, 1690, 1587, 1933, 1367, 1585, 1575, 498, 1601, 2005, 1711, 1948, 1991, 1580, 1704,
        207, 1560, 1867, 1600, 1594, 1930, 1541, 1832, 1613, 1599, 1757, 71, 1534, 1940, 1982,
        1960, 1530, 1908, 1857, 1410, 1987, 1526, 1546, 2002, 1923, 1972, 1752, 1984, 1754, 1916,
        1942, 1980, 1608, 1398, 1438, 1955, 1968, 1799, 1976, 1847, 1775, 1904, 1983, 1945, 1554,
        1486, 1527, 1884, 1553, 1736, 1561, 1513, 1695, 1431, 1997, 1405, 1872, 1434, 1679, 1609,
        105, 1582, 1795, 1826, 1886, 1472, 2007, 1617, 1978, 1669, 1764, 1865, 1773, 1993, 1666,
        1583, 2009, 1969, 2001, 1659, 1833, 1713, 1893, 2000, 1520, 1652, 1437, 1556, 1633, 1386,
        1819, 1973, 1426, 1975, 2010, 1863, 1593, 1996, 1796, 1986, 1995, 657, 1784, 1644, 1941,
        1596, 1849, 1065, 1927, 1525,
    ];

    let (a, b) = find_sum_pair_in_list(&expense_report, 2020);
    println!("Day 1 - part1 result {}", a * b);

    let (a, b, c) = find_sum_triple_in_list(&expense_report, 2020);
    println!("Day 1 - part2 result {}", a * b * c);

    // Day 2

    let input = r#"6-7 z: dqzzzjbzz
13-16 j: jjjvjmjjkjjjjjjj
5-6 m: mmbmmlvmbmmgmmf
2-4 k: pkkl
16-17 k: kkkkkkkkkkkkkkkqf
10-16 s: mqpscpsszscsssrs
3-9 n: wlxplcwkkqmdvnb
16-17 x: mdxqxcxxxtxxxxxxxxv
4-7 r: hrrrrxr
3-7 d: ddndddcddddvddd
7-15 p: rpjvppwfppsppptppqb
1-2 n: ztznhmhldp
6-18 x: stxxxxlptpxkxxxxxxx
15-16 z: zzzzzzzzzzzrzzzz
4-10 q: qqqpqqqqqqqbkdqrgqq
14-20 m: mmrmmmmmmfmmmlmmzcmw
1-2 t: qvxzmfgpxwgkt
7-11 w: wwwwwwcwwwlwp
12-13 g: gggsslggggggg
9-11 v: vvvvgvnvhvvgvv
1-11 z: tgksdczwmtzsfs
13-15 t: hrshwjqvtlcqtrtttjhv
4-13 h: hhhrxhhfhhmhhhhsdh
2-3 s: shcsssss
6-14 g: gggwgtgfpgqggwgg
3-5 t: gsmtttttcbf
8-9 b: gbbbbbbbbb
8-9 f: ffqfffflf
13-14 f: ffffffffffffnf
13-14 v: vvvvvvvvvvvvhv
2-8 m: mztxflwwrc
1-2 d: ddddpfj
2-7 f: fgffffgfffr
3-5 t: htjxtstvtspttlxk
15-16 f: ffffffffffffnfgtf
3-11 p: pbgllspgppqhp
1-10 t: mssrtttpzl
3-4 q: qqcv
4-5 s: dsscp
3-6 n: nrjnnk
2-5 z: zzzzz
4-6 d: ddddwd
4-8 f: zfqqdrfwcng
2-6 s: ssnscs
4-8 z: fzzzzvgbzwrzqzz
14-18 z: thmkmmggvkfzbzfzvzt
2-8 h: hhhhhhhh
7-9 p: ppppppgpll
1-2 x: cxkcrk
1-2 h: lbrhhp
6-9 k: kkkkkkkknk
3-4 s: scjs
2-3 s: fdshfsh
5-15 s: vssspssssvkrlhk
6-8 x: xtxxvxzhxxxxtxl
6-15 c: ncccccccjccfclccv
3-4 x: xxfcxlxxcxs
3-8 x: xwxxxtkjxmtlxxxmx
3-5 j: jjsjljzxjl
3-12 n: nnlmnnnqgnmgs
3-6 w: gwvswwww
9-12 l: hlljlcklrcls
2-4 x: cxfxqhx
6-14 m: zrgnffnkcmntszgvnc
1-6 g: gxqrgnzd
4-9 z: zpzqzzgzzxxbzs
10-13 l: jlvlllbxlvllt
3-4 n: jxnx
8-10 c: sccccczckc
10-13 j: jkfgxjjjjjjjjjxzdh
9-12 h: hhhhhhhhhhhhh
6-14 p: hvpppxnpkpjbxpppppgp
4-9 s: ssscsslsc
3-7 c: cccccgcclqg
19-20 l: ltfjvlllxhlnnlwlllfw
7-8 n: nnnnnnnnjnn
2-5 k: skntkhxkbwj
3-6 z: pnzzzzz
5-9 g: gpqtgggwxmghvgrqg
3-6 k: khkkkkkk
5-11 l: zlllljllnllllljhb
6-9 l: lllllllll
14-15 g: ggggggggggpxgkhg
4-6 n: lgmntnxshxwpnq
1-2 s: sxnst
13-14 z: pwzstpdztzhwxk
2-3 c: wmqrccn
13-15 x: xxxxxxxxxxxvqxgxj
6-8 b: pbbbbbbb
8-19 f: prbsvkqfcdnrwhlnwfg
7-8 x: sxlwxqxx
7-8 p: xdtphpppqpppp
1-5 p: cgplv
4-11 k: kqnqkfkfxwk
9-13 m: qhmnmsmmgmmmlm
3-5 z: rzzrjxcqpl
2-7 z: sjzcgsqqtfhnf
4-7 c: vcpvkbbbd
10-16 h: dspcrfnrthhvgcmh
18-20 b: zbbbbbbbbbbbbbbbbbbm
3-6 b: bbbbbbbbbbb
2-4 k: kzcmhsv
5-14 j: djcsjqjppfjwsjb
16-20 g: gggggggggggggwgggglc
3-4 g: gqggt
4-6 l: llfjwr
7-11 v: vvbmvlvvhbsvx
1-18 p: nvwrgkkhschlrbbhkjqm
11-16 z: zzzzzzzzxxfnzzzqzzz
6-12 f: bxfsflvhfdrgftfsxvf
5-10 r: trzwrbsrqr
4-8 g: zfphgnjlgsbc
16-17 v: vvvvvvvvvvkvvvvkv
12-13 n: nnnnrvnknvnpdn
5-9 x: xxjxxxxxw
1-3 z: szxzz
3-4 c: cccxc
12-14 p: ppppppppwppppppppp
19-20 p: pppppppppppppppppppt
9-10 n: nnmnnnbnnnrmnn
6-13 d: dbzdmdddvmdctwdbt
5-9 j: jjjnjljjngkjjj
5-8 q: qqqqhqqpp
6-7 t: tssbttq
13-14 r: rrrrrrrrrrrrrr
2-6 w: wxwwmwww
5-7 w: fwwlwwrw
7-15 v: vvvvvvqvvvvlvvqvvsvv
11-15 z: zzzzzzzzvzzsgzzgz
8-10 f: ffffffffffffff
9-12 p: pmrppppppppw
3-5 f: kfwqw
2-3 q: bkbqw
9-10 m: mmmmmmmmvm
5-17 f: wfqfnffffjwfvfffjbf
9-10 m: mkmmpmmmbkmqmmm
2-5 h: hhhthhk
4-13 r: cklprhtqglcrhhrmqsc
1-3 w: lwqw
5-7 n: lmxnjknnlnnvbqw
4-10 z: dlcpzqvjjq
10-11 r: rrrrrfrrrrr
2-7 h: hqtmfptxhk
11-13 g: zmdlpghbsggbglxcjwc
1-3 c: cnch
4-7 k: kkknkkkk
4-5 f: jwtffcfdff
1-10 j: tjjfjjjzjbjjwr
11-12 j: jjjjjjjwjjjjjjjjjj
13-15 x: xxgxxxxxmxxxbxcx
11-12 d: dcdddtdddddcdddj
8-10 k: kpkkkkqkmkk
5-6 h: mdmhjhhrhhdhhh
13-17 g: ggggfgkngpggrgpgclg
14-15 b: bbbbbpbbbbbjbsb
1-7 x: wcqddxxhgmxxxvn
8-10 l: lllhqlznjlllcll
6-8 n: rnnnscnq
14-16 m: fmmbmmmgmmmmmpmm
5-6 c: vccccc
7-10 h: vwgnhqhqhgpchhwhhhf
2-6 n: ndwnnn
1-2 v: vvvtr
10-11 l: lllllltlllgt
12-14 x: xxxvxxxxxxxxxx
5-7 n: snnnnnnh
3-5 l: rllmn
9-12 s: jsssgsssftsvsss
1-4 r: rfrrf
6-8 r: rrrrrrrr
3-9 g: rgpdqgsbfggflgbp
1-15 t: tsbtrptnrtqtvtlgpt
8-9 b: bblbbnbsbb
11-14 g: ggggggggggwggcg
4-5 c: scwgs
8-10 d: ddkdxddddq
2-4 m: gmmm
3-8 k: wrkjkczkkgc
3-4 h: sjhh
2-13 t: tvttttttkttttttxctt
5-8 p: plzpppppp
6-7 q: qqqqpqxq
14-18 r: rrrrrrrrrrrrrrrrrrr
16-18 w: wwwwwwwwwwwwgwwmwd
4-10 s: ssssjssshss
7-8 f: fffhffgcf
2-11 s: sdszssssqsntsz
2-5 z: zzjjzw
10-11 k: kkkkmkkkkzkk
4-5 l: lllcdll
1-4 q: qlrt
7-8 c: cccccscc
9-10 k: sxgqvkkkkkkkkzkk
5-9 n: nnlkchnnnnwjn
6-7 v: vdwvfdcbb
3-5 q: bmqwtqkdbxzdm
3-7 h: hhbhhhhhhh
14-18 q: qqqqqrqqfxqqqqqqqq
4-12 w: ndnwwcvfthhn
4-5 b: bgbbn
9-10 j: tjjjgjjjgjjgj
3-4 s: svsm
1-2 q: xfqqt
7-8 p: fdpjsvps
4-6 x: xpxxsxljlxxrxqjxp
4-8 w: mbvdkwwwwwwhghlhj
1-3 g: fngjxdc
2-3 j: xtjjj
4-9 j: lqphjhmgzjzjjvjjtv
6-9 z: szzxzzqzzz
5-11 c: cdccncccwfxc
8-9 t: bttttttvtt
5-11 l: rttpvrclsqfk
3-18 b: hlwzvtqbbzbbbbvlvhb
15-17 f: fffkffffffffffbffff
2-6 l: lllqlt
5-9 h: hhhhjchhhh
3-9 x: xlgxxxxxr
6-10 n: nnwxmxnpnf
2-3 l: drlwrtltw
1-2 b: gxbbrklvbsbbxbbb
11-13 q: qqlvqqqqqqqqqqqqfp
1-12 t: thdtvlgbstttmhgttbnt
2-4 l: lllpvll
3-16 t: lttpfnrgwdlkqtwgqrj
7-8 d: ddddtdddd
10-12 w: wwwbwwjwnwqwbwswwwg
2-4 c: qqct
1-4 j: jcjrjj
4-5 l: lllbll
11-16 g: ggkrgswgljjggmpzkg
7-11 b: rbrbbczsbbbbbbfvck
16-17 r: xfnpkmnzmlnrwrrrr
8-9 x: xxxxxxxjz
6-10 p: dpppdptpbpxpgbpppn
4-5 s: scvcjsqssbs
3-4 k: kktv
8-10 f: fffffffffs
3-4 g: gpbq
10-11 x: xxxxxxzxhgdqxxvxxxxx
7-8 g: nzgpgxgv
4-6 n: nnnbzsnbh
3-14 l: jdllllltlfjlld
7-9 v: vvvvvfvvv
5-7 s: ssbsssxjv
6-11 l: xlllslrnlflnqhklllll
14-18 x: xxxpxxxxxxxkxxxxxx
10-12 x: xxvqxxxfxjxxxxxqp
4-6 h: xfhhhhhhhj
2-3 b: rbbbjmb
7-10 d: dddddddddd
15-19 g: ggggfgggjgggggggghg
1-4 t: stth
7-10 r: zskrphxkhgtmnsmzlr
9-15 d: tkbdgngsddddrcd
3-10 q: krqsqljqdfzqqq
4-5 t: tztxtrgt
11-14 z: jzzzzzzzzzzxzz
18-20 p: pxpppbpppppppppppppv
10-17 t: tttttjtttdtvvfqttxft
8-15 j: vbjcjtdjzsjjjwjj
4-7 d: ddddddd
5-6 h: vmhlhj
9-12 h: rhhrbhhhhhjh
3-5 k: lkbkk
6-11 d: dkfsjdmdddn
7-11 d: dddddrsdddqw
2-4 n: wnxngd
12-16 c: cccccccbccccccctcc
7-13 f: blfjmffffffwlf
14-16 s: zsssssssrssssslrzfps
5-6 t: tkkttt
7-13 j: xjkjjrjmxjfpnvxqj
8-14 c: cccccccckccccccsc
11-14 k: kkknkkkkkhkfkxnkck
4-5 d: ddddd
4-5 w: wzmwwwww
12-13 r: rnrrrsrvrrsrrrrrrr
6-18 p: ppjppppppxppppppjppp
1-5 m: gzhfjm
2-5 s: scsrsssv
7-10 t: tttthmztxbt
3-8 c: crccmpwc
9-11 f: ffffwmvffzfrzkfff
9-10 q: vqvjtbtqkcnsbmfl
4-10 l: lltlnfzxglwfxpgwntj
2-3 c: pcccc
5-11 s: jjgxwdfnsgjgxk
3-8 d: vjqsdvjdhsx
1-11 t: bjpthttlzttpbxtds
7-8 s: sssssxsss
2-6 b: qbzglpffcldtdb
5-7 c: cccccvc
5-6 f: fvffffz
6-7 s: sspssdzhswsn
1-6 l: wgdppqwnqp
14-15 b: bbbbbbbbxbbbbbbb
2-11 w: vqwtkmvbgrp
5-7 w: wtwwwwwwwwww
4-5 s: rsssss
1-4 t: bwttt
9-12 m: mmmmmfmmmwmn
3-10 s: gmgjdslsnwsrv
1-4 c: zsfcsrvsxczrgbtpwjkv
11-12 f: fgffgtrfffksfff
6-8 g: gggglbgg
7-10 c: crcjtcccbh
6-8 t: vjdktttttzcnthtwvtvb
7-8 h: hchhthqhh
4-10 k: hhkknpdlfk
3-4 r: rrsjr
7-11 g: ttrjggkdwbdnzmwzd
5-8 b: nvqpcvfb
6-9 b: bnbbbftbmbq
1-4 m: smmmmmmmmmmmm
16-17 p: pppppppppppphpppp
10-13 f: ffwffffvmthvp
9-12 n: nnvvcvmmfnnqnnsnnnn
2-4 r: rrwq
2-9 x: xzxxhxkxxtzdc
6-7 t: ttrtltxt
4-5 f: fffgxf
3-6 v: vvvvvm
5-11 m: mgxzmmgmlxxmm
3-4 n: knnjcxn
3-4 d: dndkgvkpbhdddcpt
6-7 x: nxxxxtd
11-12 k: kkkkkkkkkkmkk
7-8 b: bkbbbbbzbbfmj
2-5 d: drtdq
8-9 q: qqqqqqqzj
7-9 m: mkmmmmzmpfm
3-5 b: bwcbv
2-4 v: vvvvvvv
5-6 h: hhhbbv
12-15 g: gggggggdgtgqggngg
8-9 l: llldllllll
3-11 f: fdfpflzwfffct
5-6 z: bhbzjgzz
5-18 r: jtmnrhfrsptklgdksr
7-9 d: llcrcqsfc
16-17 l: kllllbllllllllllll
9-15 g: ggvtgvgjggmgwslggm
4-10 g: ggggggqgggqggp
3-5 c: gchcc
6-11 z: zzzzjzzzzpz
3-16 h: hzhhhhhhhhhhhhhhh
3-8 w: wwqwwwwdw
3-4 g: wmklgdxxb
2-5 p: ppbwqsg
1-3 p: xpvpppppp
13-16 x: pfqxdnxwxxzpxxvx
3-8 m: qmlhjkzhm
8-10 l: llllllfllll
12-16 p: xtppfpfppppppdlk
7-10 t: ttttttztmt
11-13 p: pwlphpppcwnpv
1-3 x: xxxx
12-14 c: hcckccccccbvdz
3-8 g: ggggggngg
11-13 b: bbbgbcbbblxbbbbbb
2-6 k: kkhhlhvtbd
3-4 s: sstsss
13-16 l: nljgklqlgfrlljtl
9-14 l: llllllgllllhln
3-15 b: bbtbbbbbbbbbbbpbbbb
7-10 h: hhhhhghhhjh
4-6 r: pnxrvrrnwrcrrw
17-20 h: xbhcmhpvqzlhrbhhhbjn
1-4 f: fffff
2-4 c: vcclc
11-14 f: fffgnffflffffff
8-15 s: ssslsscwcssszsv
7-10 r: drrbvtrphzqqrrsvncr
12-13 l: vllllllllllbfl
7-11 z: zwhzzhmqnbj
6-11 v: vvkvvqhvvvjvwvvvvvvv
13-14 h: shshhmhhhhhhfhvsr
5-10 c: rwzcxwzvgcpn
4-7 f: ckdfpmffpzlwjl
5-7 x: xxxxkxgxxx
18-19 x: xxxxxxxxxxxxxxxxxkxx
4-6 r: nrcrdgrbrb
9-12 f: ffffffwflffff
5-10 z: zzzzzzzzzwzzzz
4-5 d: dqmvqlhjxdkkdfn
10-15 s: ssssssssssssssssd
8-10 w: wpjvmwqwwz
6-10 w: hxwxcwrqbb
9-15 f: fffffffffffffbftb
5-7 q: qqqqqqx
3-5 t: wgbttttp
12-13 k: kckkkkkkkkksztk
12-15 w: wgwwwwwwbwwwwww
2-3 s: tssstcph
4-7 l: llllkll
6-7 w: wkwkrxsj
3-4 w: wkwwfm
3-10 w: swtwckrrkwrvk
5-6 w: wwwwwwlwjkknh
1-3 d: dddd
1-2 b: bcbj
8-10 f: fffffffbvbf
7-12 d: ddgvftsdrffbtd
5-9 f: fffffffffffftf
4-11 j: rbhjbkfjznjwqwz
3-9 z: czzgzbhvz
9-12 x: xxxpxxxwjxxvx
3-9 k: zzkfjkcfk
7-9 f: tffqcfgfvlffmd
2-4 j: njzn
9-10 h: hhhhjvhhhhh
3-7 n: hndnbjdjx
1-2 s: sshfsss
7-14 g: kglngzbggggnvcgtgt
11-13 h: hhhdnfhhhhqhhhhh
5-6 w: twwwww
5-8 j: fgxbqjjh
7-8 f: fffffhvp
3-9 j: pjjjjjpbj
3-4 m: vmmsm
12-13 x: xxxxxxgxqxxjnnx
4-5 t: ttbbtt
10-12 p: jjpppppcppppblpp
9-10 b: wlbbbbjbbkbgnbbb
6-8 t: tttstttzt
6-9 n: fqxlnnnmn
4-5 k: gwdzkk
4-11 t: ttthktthtttbt
2-4 r: rrpcnhrktrl
7-9 j: hjbxdqbbmxmm
3-7 b: bbbzqbbsbh
15-17 s: sssdssspssssssgssss
6-11 w: wwwwwpwwwwdww
14-16 s: sssssssssssssgsrs
2-8 z: kzbzbzthzgzzwcw
11-13 q: qqqqqqqqqqjwq
3-4 v: dspkvr
1-5 n: nnnnnnmn
4-5 b: blbbsbb
5-7 r: rrfrkcr
5-6 c: zpcrlp
15-16 b: bbbbbbbbbbrbbbbg
5-6 j: jjjjrcjj
9-12 z: mzgnzzdjzzzzzdzlzzzz
5-7 z: gztczpzbzdzlzcjzj
6-8 w: hwhwfmswrrcpxlsqlkkd
6-11 p: pppsppmpnpp
1-2 w: vdwl
8-13 m: mmkgmmlmnmmvm
2-3 q: qqqh
2-5 x: xxxxrmxxxxxjxnxxcnd
7-8 p: fwpppptp
2-10 l: lllbhlltclln
10-11 v: vvvvvvvvtrgvv
13-16 m: mmmmmmmmmmmmnmmmm
3-4 n: qtfn
7-8 g: gggwgggw
10-12 n: xnnnhcqhnnnpnphnnnnx
3-4 q: qrqq
4-8 c: cswcccclzx
2-8 h: hphhhhhd
1-16 r: qrfrfrtgrrrxrrzhbg
11-12 q: qqqqqqqqqqqqqqqq
10-11 v: vvvvvvvvvdf
6-8 n: lnnrnnwnnldmwn
1-5 j: jjdkfgkjvjjjjj
14-15 j: jjklzggckqksgvk
10-11 g: ggggggggnzng
15-17 n: ntnnllvnmbzwnnhzw
13-16 g: gggggggggggqdgghc
12-16 g: sgggggggglggnggggggg
3-4 x: xnplx
5-9 m: mjldmlmmmgbmjsbkm
18-19 h: hhhhhhhhpzhhhhhhhhh
9-10 f: ffbfffdfjf
8-13 s: gssghjssgxsgssgsnj
2-5 m: mmmmbhdzkmmh
8-18 l: hqnqzznqkmskljggbl
2-3 m: nfjm
1-7 z: dnztzkpfwv
2-9 s: ssvsdvjwdsw
15-19 f: fffffffffffffcsffcn
13-20 c: ccccccccccgccccwcccc
3-15 b: bwmzdgbbjvbbbrt
10-13 d: ddddddgcddmdhdd
4-7 h: hlhhhphz
2-3 c: cccbgfbzdql
13-14 w: wwwcwwgwwwwwcqwzw
2-4 m: hmmlmmmmmm
3-16 x: jzxcfkxzxcvfxwxfxx
3-8 f: kfpffvfwfkf
8-10 q: pqqmlqqcqqqqf
3-6 r: qrfcrrwqcrrbttrb
3-10 s: kdstdvqqdcb
11-12 f: lcxfllffffffflffff
4-8 m: mmmmmmmzmmmmm
6-8 w: wwwwwswwdw
10-15 b: bhfbbjclbbhwthxltsjn
1-5 v: dvxvw
13-16 s: ssssssssssssksswss
6-7 d: dlfldrt
4-10 x: xxxxpxqxgxgrh
4-8 x: xxxxxxxxxx
4-5 r: vrxrrrrrkxrp
3-14 h: hfmpnxlklhhhjhzh
11-15 d: cgddqnxgddddzrdd
7-12 v: csmfvjvlvzjv
4-5 g: ggngh
14-17 z: vzzzzzzzzzzznvzzzb
15-20 q: qqqqqqqqqqqqqqqqqqqn
1-3 x: xbxqnzhjzhbxbbnsnt
11-13 d: sfdtlhdddjkshd
11-13 c: vbtcncclfzmcc
1-7 t: ptttttqtt
3-4 x: xxsxx
1-2 h: hjhh
10-13 f: vlcfqpwgqfthf
14-19 w: wtswwwwwcwwpwswwwww
7-10 h: fhxhhhhlbw
4-14 k: khfkkckvkdxkfkvp
4-6 k: kknkkkbk
3-14 m: mhvdmpnmwvkcmgbbv
17-18 h: hhhhhhhhhhhhhhhhrs
16-18 c: cclccccccccccccxcmcc
9-12 j: vjjjljjjqjjjjq
2-3 d: dlrbddmj
10-15 l: lllmlllhllllllll
1-2 n: nbnd
8-11 b: wvbbbbdbbbbbpxbbb
4-5 c: mccccfcc
2-12 d: kbdsddgddpwddzx
9-10 t: ttttjttttbtt
6-7 j: jjthjvwj
2-7 l: lglqllxlh
2-4 t: tnqjb
1-8 c: nqrbcgbchxwkjrxv
13-18 c: ccxmccccccccdcgccn
9-10 s: shsssgsgssksl
4-9 n: nnnxnnzngrpxn
10-14 x: jprjxdlnrfwnqx
1-5 b: bsshbls
1-4 n: nttnmbblbntbssqn
8-9 x: xxxxxxxxxxx
2-4 c: nnccmp
1-14 t: stttztttttlttq
9-16 b: rfbmptrnbhwbrgbbqb
1-4 f: rzzfjkmnwlbnrqmfns
1-3 x: mxxdlxnklrktlgzd
3-13 v: zlnktcwvmkqsvjvnwzv
1-3 r: rrrrr
5-7 d: qdddcddldd
2-5 t: tnttktt
8-15 n: ngnnnnngnnnnnnn
1-2 v: vfvn
3-17 f: ffffjfffffffffffgff
5-12 x: nxxxxxtxxrxxx
2-14 g: nfgvdgnhggrgtkv
11-12 k: kkkkktkskkkkksm
15-16 c: rccccccccccccccccc
10-12 l: cllwldllplsl
15-19 b: qbjbpbbbbbbwbbgbbcq
13-16 f: dqcfgwfffffffffb
3-4 q: qtzj
13-15 z: zzzzzzzzzzzvzzzz
13-18 q: qqqqqqqqpqqqqhqjvqsq
7-12 r: dtflckrndscrqrcclxb
4-9 s: zqsssshqscs
2-5 s: ssrhsfsszsc
7-8 k: kkkhkkwzktkkk
17-18 x: xxxxxxrxxxxxqwhklp
16-17 k: kkkkkkkkkkkkkkkkrkk
9-11 s: szqfssvgfvm
11-17 m: mmmmmmmtmmmmmmhmpmmk
6-7 l: kgllhjdllwqvllnlcn
1-2 g: gghgfgv
4-5 f: qfrrrlfrtfzgfct
6-7 z: zvzzzzj
7-12 m: lmhzrxztktzpkvw
3-4 p: ppxpp
4-5 c: wvxch
9-12 x: xcxxxxzxcxpxx
10-11 t: tttttgttttl
2-3 w: tcwdwb
1-8 m: mnsmmmmgmm
1-5 z: zzzzz
2-13 j: jvjjjljjjhjjjrjlj
4-5 s: kcmxszckjcsswjbsxn
6-7 j: dhjccbb
15-19 x: xxxvxnxxxxxxxxnxxxx
15-18 f: ffffrffsfffffjffsff
8-20 m: xmcwmmmhsmmmmmbmmmmm
8-14 m: mmmmmmkmmmmmmm
2-6 r: bnrrrv
8-12 k: kkkkkkkjkkkpkkkk
1-4 f: flqfgnffzjrfz
2-6 p: pcvgxpnl
9-12 n: nnnnnnnnkgntnn
13-15 h: hhhhvhhhhhhthhhhwphh
6-7 h: hhhhhhkh
8-9 h: vhhhhhhhhh
1-5 m: mbmmmszqwmxdhd
5-6 v: vvmbwvv
4-6 g: vggttg
8-15 t: dzkznmcvqmbbtxs
13-15 j: jjjjjjjjjjjjjjjj
5-11 r: ptwmxlrttvxk
10-12 s: ksszsbqxsjsssnjst
3-4 k: jwkr
3-6 m: mmtkmmm
2-4 l: lhlll
7-12 p: pppppprppppzpp
1-2 n: bznpdd
11-13 t: ttttttttztdct
6-12 c: cccccdccccccc
2-7 n: nznnknrnnn
14-15 h: zqxhdhhxhhhhhzpr
6-7 x: xxxxxjx
5-7 s: gsfcshsbxnc
12-13 j: jjjjjjjrjjjjjjjjj
4-5 v: vvvvc
4-10 r: rrsrzrhrrgrmvrs
8-12 c: cccwcccccccmc
4-5 v: vvvvv
13-19 j: lfnkrsjjjjjrjjzgcmj
3-4 k: kkskkk
11-14 s: sssssssssssssssssss
8-9 k: kkbkknkkk
4-6 z: zzzqzqxzz
2-4 f: rgtz
4-13 k: kkkkjkkkkkkxk
1-6 q: jqqqqpqqqqqqqqqqqq
7-9 b: bbbbbrlbj
4-5 j: mgjjtjfs
15-16 p: ppppphppppppppnpp
2-3 b: btbt
11-12 v: vvvvvvvvvvzk
1-4 b: qbbbh
6-7 b: bsbbbbnwb
3-10 j: gkjrhtjjfjjjrjjjjrw
4-5 g: gdgqtpgg
15-17 g: gggggggggrgggggggg
1-3 j: hjdjpj
5-6 k: jjslkhnwkfqjprk
7-10 s: ssscssssnsx
1-9 n: nnnnnnnnnn
1-14 t: ptttttttttttttt
11-13 q: qqqtqqcfvqsxzqqqqqq
14-15 r: rrprsrrrqmjbrrv
4-5 h: hhhhdhh
5-8 l: lllllllll
18-19 f: fwfffffffffffffffff
9-11 j: zjnjjvjjfbj
3-7 p: sjpsmsc
10-11 s: sscsssssrrls
3-9 q: qqqqqqqql
6-20 s: sqscdsmdgvddfsvprgcs
18-19 q: qqqqqqqqqqqqqqqqqqq
4-5 g: mgjggnjfgl
3-5 p: ppcpzp
14-16 t: fjxbttlttfprttlt
4-7 v: pzmvzvlzxcccvmvvwfdb
1-8 r: rkrrtkrrgrrq
1-10 h: qhhhhhhhhdf
1-4 p: rpppp
4-6 s: skshzg
16-17 h: hhhbhhhhhhhhhhhch
2-10 m: mrmpvlsxvtlkwpnl
7-10 b: bbbbbbbrdb
15-18 j: mjjjkgmjjbjjjjcjjmjj
3-6 m: mgmmdmmmj
5-11 n: ktnnwnnntznqnnnpnnn
9-12 w: wwwwglwwzwwtww
8-9 t: ltttvttvv
4-8 c: xdfklzfhgjgcktssk
7-11 x: xxgxxxxzxmxxxxqcc
13-17 v: vvvvvvzvvvnvvhvbtv
13-16 c: ccccccccgccccccw
3-13 m: mmkpmqjkdknmmrmfcm
7-11 g: gpgsvmgvdkg
9-14 g: gvgglglghgjpwjzgsfh
2-3 v: vvcv
3-8 w: zwqjwwwwwm
7-9 k: kkkkkkzkb
3-4 v: gvvk
14-15 r: rrrkfhmpgbhtqrh
4-5 n: nnnmwnn
7-9 l: llltdlvlx
6-7 f: ffffffl
7-8 n: nmnnnwfg
17-18 x: xxxnxxbwxxbxxxggvxx
1-3 z: cbzntpzvgpszsdq
3-4 j: jjjj
3-5 z: zznzgz
4-5 j: jjjcg
7-8 j: mjjkjjxhj
1-2 t: vrlmttt
3-4 h: hhhhhrhhhphhh
14-20 q: qqlsqqqmqqqqqqqsqjjg
9-10 r: rxrrrrcxrbrrnsc
8-15 x: ghlzxrvjvjxrndvknxxt
1-3 b: bbbb
4-6 t: fnqthtnfjprghdmt
6-7 m: mmmmmcgmm
6-8 q: bqqvqvqd
7-16 v: vvvvvvvvvvvvvvvvvv
3-4 p: ppjlppppppppxlztpp
12-14 g: ggggnrggvgggmgg
1-10 k: kxkktkkkpkkgt
4-5 z: zzwrw
14-20 p: pppppppppppppspppppp
6-11 t: hkbbtbmrdlkjlthgkc
6-13 f: fpcfzfcpfjqhqwffffd
2-3 h: hhhjhh
3-4 z: ztcbz
6-9 q: kqlqgtqwvxcgqwqr
9-14 k: kkkkskkknkkckjk
14-16 n: nnnnnnnnrnnncprcnn
1-9 m: pwwmgmxmnmm
1-5 f: fnmpfc
3-5 s: jmsls
5-7 b: bbbbbbbbbbbj
4-12 m: gjzqmjktbrbk
4-5 k: krkkkk
3-10 q: qqcnrqqqbnqq
1-2 s: gvsk
5-6 n: nsjnnn
1-5 w: wwkwztqc
4-17 z: zbbnqfqmrjbszpwxz
2-9 z: djpdvrdbx
2-4 w: lwxw
1-10 g: gsgpgggrgggtgwsgg
3-6 q: qqtqqq
3-7 x: xdxxxxx
1-2 t: ttft
4-6 x: lxjxsxpvxxxkjqlsxndf
1-8 r: rbzrrrrrrsrrrrrv
1-6 g: gggggggggggggg
2-3 q: smbbmntqr
8-9 d: bzmplgdfq
5-10 w: wwwdwzjwwwww
2-11 z: jzgphzdjwxw
9-12 d: dtvdddjdlddddd
5-16 s: hswkqwssjshpsxkgmsw
3-13 c: cwbcccccccccgc
2-13 b: bhswhcxchtknt
1-3 h: hhhtmclgrtxdvqfpk
12-13 w: bswxxxmxwjtwp
4-6 c: xtcccc
6-16 k: msskfkgdkpcnqtzkpqh
6-7 t: tpttftt
5-7 h: hhhhhhh
3-8 t: tbrshhlt
8-10 j: jjjjjjjhjjjjjjjjj
11-13 g: vgwggggggkshjggnqggs
6-10 q: jqqgzqpwqqfqqqd
5-12 d: ndwgszjhhdwv
4-5 t: tttmkttt
2-3 p: ppwp
7-8 t: ttttttzb
13-14 m: mmmmmmmmmmmmpxmm
5-9 v: cqvbqvzzvtcdnvvvbz
10-12 q: xbfpjqpzlqcq
8-10 l: lhlkllclll
4-15 t: wjtqftxkskpvttxmb
1-5 s: lzscqfrpfss
7-9 x: xmvxfxxjx
6-7 w: wwwmwwwwww
13-16 d: kbddtmrdqgqqddhd
1-17 q: vqqqqgqqqqqqqqqqqqq
3-5 f: nkftsfvzc
4-7 h: hhhhhjrd
5-11 z: zzzzzzzzzzlz
4-7 k: kkkkdkkk
17-19 n: nnnnnnnnnnnnnnnnwncn
8-16 r: hpvsmthdvmxrbqvr
2-10 b: bbrxblvbbdvcbdbbbqsb
8-11 r: rrrrrrrzrrv
1-2 r: vtrr
2-3 m: mmcmmmm
3-10 c: xccccscccc
10-11 w: wwwwwwwwwwzw
3-18 f: fspffffffjpfffffff
4-16 q: kqqqqqqqqqqqqqqkqq
10-11 l: llllxlllldv
13-19 t: ttttttttttttttttttt
9-12 d: drddddddmddd
5-6 n: kngrnn
5-9 x: sbxxxtkvpjfxxxv
8-11 d: dkwddrdrdddd
8-11 d: ddsddddhdbd
4-5 l: pxlll
12-15 m: sjlnmmhndcmmzrmm
7-9 t: ttttttdtctt
3-5 n: snfxc
4-5 l: lmpll
5-10 z: zzlztzfttxzfprz
13-14 d: ddwjdqvjdddvdj
14-18 r: rrrrrrrrrrrrrgrrrhrr
4-6 g: ggggqc
9-16 m: mmrmmmmmmmmmrmmm
1-8 f: fffffpfkffff
7-8 b: bbbbbbdtbbb
5-8 k: hkkbkkkkkkwk
9-12 d: dhdddqdhddkddddddd
10-12 r: rrcrrxrrrgrr
6-7 r: rrrrrrrrr
12-18 c: cccccccccccccccccc
1-3 x: glnwxx
4-12 s: splscqfbvbvswgrvvhg
7-14 n: nnnnnnpnnnnnnb
4-8 x: xlqxbrkx
11-13 t: ttttttttttktv
1-7 w: wwgwwww
8-13 p: lpzxtppplplff
6-7 q: qqwpkqqqqz
4-5 j: zdkmjrpjlzbjprv
4-6 x: xhtxxxg
2-14 j: jjjjjjjjxvjjtjjjwj
8-9 d: gddddndcgtdn
4-7 g: gglgpgr
7-10 d: drddddsddd
1-3 n: nbnnb
3-4 k: dggss
6-10 t: cwcwtnttmt
3-4 x: xxxx
8-12 t: jttmtbttqtztttzmtc
8-10 t: tttgtttztttg
3-4 h: hhhhc
10-18 t: tttttttttmtttttttbtt
3-5 n: nnnnnnz
4-12 w: wwwwqwwwjwwxwlwqww
2-4 x: xqdpr
9-16 k: kkkkfzbkkkknjknkkr
2-7 t: tttttwt
2-4 p: npjp
5-12 c: cccccccccchxcc
9-14 x: xvbxpxxshxnmxzxxxwn
10-11 t: ttqtvttjtcttmtttt
9-15 c: cccbcxcswnrzchfcqwjc
3-4 s: vpczjdzgfsqnpsscf
7-11 b: bbbbbbtbbbb
4-15 q: qqqzqqqqqqqqqqqqqqqq
2-4 s: fsvsmlsfbgz
1-8 w: wbwwwwwtww
8-9 w: phbxcfdwwhrbhkgrzdx
6-9 x: xxdpxhsxxxkx
5-9 b: bbbbfbbbsb
12-18 m: mmmhmbmmmfmrmmmmmmmm
12-14 l: lllllllllllllll
8-11 z: zzzzzzzzzzzzzzzz
2-3 d: dlld
6-7 b: kghbbbbbg
18-19 c: cccccccccccccrcccccc
6-8 m: mwnpmmjmfmmmfmhjf
1-5 j: gjjjxl
3-4 p: pzpppppp
12-17 z: zkzzzpzzzzzndzzzzzz
13-18 k: knvkkkzvkkkkkkwkkmkc
3-4 h: hfgrm
2-6 z: xzzzvzkzzzzvprtgz
12-19 b: bbbbbbbbbbbpbbbbbbdb
3-11 v: xvrvmvvvfgzvjv
7-10 r: frrrcrrrrrrzr
1-17 v: vhpxjwbpkcctdpsvfkxs
3-8 d: hbdzlkdk
5-6 f: flffzf
2-4 s: ssnsssh
4-5 m: qmffj
6-11 z: zzzqztznfzzkc
3-4 m: tjkm
2-4 n: nnwn
4-5 s: npxck
16-17 n: nnnnnnnnnnnnnnnpk
10-12 s: ssssssssssss
2-4 g: twghtgs
9-11 w: wwwwwwwwdww
15-18 h: gfvmkzsrczchxwfvdr
9-10 p: ppppppprtp
4-5 p: pppgf
2-11 c: qccccrsbjbsxbnjx
8-9 l: lllllllvl
10-15 s: sksssssssssscssss
4-8 k: kfkklhgk
5-9 t: ttgndtthqcfj
2-3 t: spntttnqdttjt
12-16 t: tttttttttttttttttt
19-20 j: jnjdjbzjpjjnjjrjjjdj
2-6 c: rrbhhggnnzcjvp
7-13 r: rrlrrdrrrrrrr
2-8 f: dfsvpmqfbqc
10-12 p: bpprppnbprpppp
4-6 z: lzszznzzxxzzm
2-5 v: vmvvv
9-17 v: vvvvvxvvvvvvfvvvf
6-10 t: tcflkttctht
10-16 d: qbxwwbqdxxlqtndktlx
1-8 g: ggtgggcgggggr
1-2 t: tttqqt
9-10 k: knkkcnkdkkzkkkq
12-18 r: rrrrrrrmrrrrprrrrkrr
2-5 x: xxxcxxxxxdsjx
6-7 z: cvzzzzz
5-8 b: qjpzdbdx
19-20 n: nnngnnnhnnnnnnpnnnwp
3-19 g: vgggpxgnfkhvdrgggsng
3-5 b: bvbbqh
3-8 h: hhghhhhhhhhhhhhh
2-4 x: xxtxfcppsvxrdtnjgs
6-8 x: xxxxxxxx
1-4 k: kfzkbkkkt
17-20 h: hhhhhhhhhhhmhhhhphlh
5-7 b: tfbvbrc
2-4 f: ffcgfzfrmfsft
18-19 t: ttfttqklqvgtkltnlgn
1-4 c: tctc
8-18 g: gkndqqgdgqrgkzgggg
11-12 b: bbbbbbbbbbbpb
5-9 t: qgtntdbdw
1-5 n: nqnnnnkns
1-3 c: bcccqccccw
8-13 l: lllllmpkllllwhl
7-10 c: czcsbcnccqwcsgxpcw
3-5 h: bhhqjjpnh
9-10 p: pppppppppw
4-5 x: xxxxf
1-5 h: mhhhhhh
1-17 c: hcccckccpccccfccc
8-9 p: pdgpplpppp
1-7 r: rrfrrrmrrr
3-18 x: xxxxxtxxxxxxxmxxmlx
1-4 t: fttt
9-14 x: fxxcxmzqgzzjktddmcf
3-4 t: tttt
3-5 l: lljmk
1-3 g: dswcw
7-8 l: ltlxlnllll
2-4 q: lqvqpjcqfk
2-7 r: prrrjbrrlmrbb
4-7 b: bbbbbbbb
5-6 k: krhkkkkkkk
2-6 x: xxxxxxx
6-12 t: xmttzbtspxgd
1-8 x: xxxkxhxxxxx
7-12 r: rrrrrrzrrrrq
17-18 k: kkkkkkkkkkkkkkkkvsk
12-15 j: zjbctccpfljrzbd
9-10 z: zzzzzzzzzmzz
1-4 l: crmn
6-15 k: nkknzkkrgkcnkds
3-9 z: zzzzzzzzzz
2-10 v: vvvtpkxkvvvvzvvvvvvv
1-2 h: hhhsn
2-4 p: qgpgp
2-4 p: dvqjxdwnvrkpg
15-16 p: nkprmxpxpxhvpcmw
16-18 b: qbbbbbfbbbbbbbbnbd
4-5 t: thtxt
2-7 g: rpvlhbdw
6-7 k: kkpxdkg
7-9 n: nnnnnnqnz
17-18 h: xnmkfwtzflhkpsjthhbl
3-5 k: kkdkm
14-15 h: hhhhhhhhhhphhhchzzt
6-13 j: jjjjjjhjjjjjjjk
2-4 x: xxxwxx
1-2 s: ssws
4-5 m: fmztq
5-6 p: ppppld
3-7 w: wwwwwwww
5-7 d: vgfddxdzvkndndzm
1-10 n: nnnnnnnnnnnnnn
1-9 q: gwmrrblpmqlqpqgqqmqj
2-12 s: mqspzltmqsngwmlmk
3-9 w: twhfxnkmw
12-13 c: cczcccccccccl
2-4 d: dpdtdqlssxddddddhd
1-2 w: jqvw
6-14 n: nnnnnjnnnnnnnnn
6-7 b: bbbbbbbb
14-16 z: zbvzzzzzzzzzzzzzzzzx
7-12 f: fffnfffdfqfffg
1-2 t: ftttt
5-18 d: dtcdddjvdxddsdtdkk
15-18 s: szsssssdsssdhstssp
6-9 m: lmmmmmmmgljm
2-6 n: nvsnjg
5-6 l: lllhvx
16-18 r: rrrrrrrrrrrrrrrhrr
1-2 b: qbkbb
10-18 x: gfjmjtbxxlxmxzxxql
2-5 c: cdbjm
3-4 k: rsvvwknnkjvwnkkf
1-3 b: mbbbbb
3-4 c: ccccc
3-4 x: zxtkxn
2-10 d: dbdddqtdxvjq
8-11 k: kzkkpkkbkzq
4-7 h: hhxhzmwhhhh
11-16 r: rrrrrrrrrrrrrrrlr"#;

    let mut policies = Vec::new();
    let mut passwords = Vec::new();

    for line in input.lines() {
        let res = line.split(':').collect::<Vec<&str>>();
        policies.push(res[0]);
        passwords.push(res[1]);
    }

    let mut count = 0;

    for (&password, policy) in passwords.iter().zip(policies) {
        if is_valid_password(password, policy) {
            count += 1;
        }
    }

    println!("Day 2 - part 1 result {}", count);

    let mut policies = Vec::new();
    let mut passwords = Vec::new();

    for line in input.lines() {
        let res = line.split(':').collect::<Vec<&str>>();
        policies.push(res[0]);
        passwords.push(res[1].trim());
    }

    let mut count = 0;

    for (&password, policy) in passwords.iter().zip(policies) {
        if is_valid_password2(password, policy) {
            count += 1;
        }
    }

    println!("Day 2 - part 2 result {}", count);

    // Day 3
    let input = r#"...........#..............##...
...#....................#......
.....####...........#.#..#.#...
....##.#.......................
.......#.##......#.###.........
.#.....#.......##.......#.....#
...........##....##.#....#.....
......#.........#....#.........
..###....#.........#....#.#....
....#....#.#..#..#.........#.#.
..........................#...#
.##...........#...#.#.......#..
#....##...#.#....#.............
....#..##......##..#.#....#....
#..........#.............#..#.#
...#.####.....#..#.#.#..#...#..
..........#......#........#..#.
............#.....#..#..#....#.
.................#...#.........
..#...#...................#....
..............##...#...........
..........................#..#.
#...#...#............#...#.....
.................#..##.......#.
............#....#.............
.#......#.#...#....#...#.......
.....#.....##..##.....#.......#
.#..#..##...............#..#...
#...#...##............#........
.......#....#.......#..........
...............................
#................#...#.........
...#...#..#..#.............##.#
......#........#..............#
...#.....##.#...#...#..........
.........#..#........##.#...##.
#.........##..#.......#........
........##.#.#.................
.#....#............###....#....
...#.##....#.....##..#..#....#.
....#..#........##..#...#..##..
..........#............#.......
.........#........##....#..##..
#....#.........#.#.......#..#..
...#....#......##.#............
........#..#...............#...
..............#.....#........#.
......#..#.#........#..#..#.##.
..#........###....#.#..........
...#..#...#.#....##..#........#
........#..#..............#....
#.####.................#....##.
.#................#............
....#....#....#................
#......#........##....#...#....
......#..##..#..###...#.#.#....
.#..........##.................
...#...#....#...#.....#.....#..
............#......##.........#
..............##...............
##....#....#...#...#....#..###.
...................#.......##..
#.....##........#....#.........
...#.......#...........#.......
...............##..............
##.......#......#.....#........
#....#..#..##..#.......#..#..#.
.....#.............#.......#...
......#..#........#.......#.#..
..#...#...........#.##.........
..#................####.#..#...
......##....#.........#........
..#..#.......#...##....#......#
#.#..........#..............#.#
.#.#..............#.##...#.....
................#.....#.#......
##.........#.........#.....#...
....#.#.....................#..
..#..#..#........#.......#.....
.....#..#.#....#....#.....#....
..####....#.#.........#........
#..##...##..#.#............#..#
.#........#..##.#.....#......##
.##.##.....##....#.#...........
....#..#.#..##............#.#..
........#.#...#....#.........#.
.....#.#.#.....#....#.....##...
#...#..#....##..#..............
..#...#....#...##..#.......#...
.#....##.......................
.........#............##.#..#..
....#................#...#.#...
...................#..#...#....
#..#...................#.......
..##..............#..........##
...#.##......#.............#...
.........#.#.........#.........
...###......#.................#
..........#....##..............
.##..#....#.........#.#........
.........#.......#.......#.#...
#........#............#......#.
....................#..........
.......#...##..........#...#...
....#.#.......#.#...##..#.#....
...#..........#..............#.
........##..............#......
......#...##......#....##......
....#.....#.#.##..............#
...#...........#.#.............
...........#......#.#..........
...#.#......#......#...#...#...
..#.......................#....
...#...#..#..................#.
##.....#.....#..#..#.....#...#.
.#..#.......##.#.#.............
......##.......##............#.
.......#..#..#.......#....#.#..
......#.....##..##...#........#
.....#........#.##..........#..
#....##............#........#..
.....#..#...#............#...#.
##.#....#........#.............
.##...............##......#.#..
###..#..#.......#.#..........#.
.....#...........#...##........
..#.#.#.........#.....#....#...
.....#....##.......#..#.#......
......#.....#...#..#...##..#...
.....#....#................#...
......#....#.#...##......##.#.#
.....###.............#.........
.................#......#####..
.#.......#..........#.#....##..
..#..#.......#.....#..#......##
..........#.#.##.......##....#.
##...#...##.##......#..###.....
..#..#..#......#....#..........
..#...#....#......#....#....#.#
.#...#........#.....#......#..#
#.........#......#.##.##.......
#.##..#.............#.....#....
....#.......#..#..##...##......
...#.............#.#......#....
#.....#..........##...##.....#.
...............#........#....#.
#.....#...#..#.............##..
.#....##.#.......#.#..........#
....#....#.#.....#....#......#.
......#......#.................
.#.#..#.#.#...#...#..#.##.#..##
.............#.....#...........
............#...#..#..#.....#..
.#..........#.......#....#.....
......#..###.#...#.............
......#..........#.............
....#.................#..#.#.#.
...##.##.#....##.##............
####......#........###......#..
..#.......#.#..#.##............
.....#.....#.#.......#.....#...
.....#..........#.#............
#.....#.............#......##..
......##..........##....#......
.#..............#..........#...
......#..#...#........#..#....#
.#......#.......#..#...........
..#..#....#.#.......#....##..#.
........#.#................#...
#.......#.##.#......#...#.....#
..#...#.#.....##...............
..........#.....##.............
.......#............#........#.
...#............#......#......#
.#..#.......#...#...#..#..#....
#....#.#...#......#...#......#.
.#.......#..#.#...........#....
...##.#...#.......#..........#.
.....#..............#..#...#...
...........................#...
.............#.....#...........
....#.#..#..#...#..#...........
.....#.#.#..#.#....#.#.#.......
.......#..............#.....##.
........#..#..#.#..#...#.#.....
.....#.#...#.#.#.....#..#...#..
.....#....#.......#......#.#...
.#.#...........#........#......
.##..##......#......#......#.#.
.....#.###.#.......##.#..#.....
#.......##..#.........#....#...
.#.............#.........#.#.#.
..........#..#..#....#....#....
#....#...........##..#.....#..#
......#....#...###..#...#......
.....#....#........#....#..#...
...##..............#.##...#....
.#............#........##......
..##........#.#...........#...#
..#.#...##...#..#..........##..
.................#.......#.....
......#.....#............#.....
.#.....#.........#.#..#.#......
.............#.#.#..#.......#.#
#......#.....#..##...#.......#.
.......#.#..#...#.........#....
...#..##...#.........#.#....#..
........................#..#...
....##..##................#....
.......#..#.......#........##..
.....#....#.##....#............
.#....#............#.....#...#.
..##.....#......#......#.#....#
...#...........#...##....#.....
......#.##.#..##...##.#.#..##..
.......##....#......#....#.#...
.....####..#............#..##..
......##..##..##.........#...#.
.#.#...............#.........#.
......#......#...........#.....
.....#.......##.....#..#.......
.....##..#..#....#.#.......#...
...........###.###.##..#.#..#..
.#...............##.........#..
......##..........#..#.....##.#
.............#....#....#..##...
.#..............#........#.....
.#..#.........................#
.##..............#..........#..
..#..#.#.#.#......#............
....#...#.#.#....#........#..#.
.....#........#....#.....#.....
.#...#.#......#..#........#.##.
.......#.....#................#
.#.#........................#..
............#..#.......#.......
....##.#........#...#.#.#.#.#..
.....#.......##................
...##...#....#.....#.#.........
#...#..............#.......#...
...#.#.#.#..##....##...........
.....##...#....#.....#.........
#......#.....#....#............
....#..###....#.##.......#...#.
..................##.#......#..
.....##..............##.#....#.
.........#...#........#..#....#
.##..#.........#....#..##...#..
#.#.##................#.##.....
..#.#....#.#.......#....#......
..#.#.##.#.......#.............
..#....#.#..##.#..........#.#..
#.....#.....#.....#.........#..
#.......##.....#....##.....#...
..#...#.........##.#..##.......
..#.#.........#.......#........
#.....#.....##.#.#..#...#..#.##
.........................#.##..
..#.#..#..#..#........#......#.
..#..............#.............
.....#.......##.##.....#.......
....#...#...............#..#...
....#......#.#........##.#..#.#
....................#..#.......
.....#.......#......#.##.......
#.......##..........#.....#....
.#.......#....#.#......#.......
......#...#...............#.##.
....##.#.....#.............#.##
#..#................##...#.....
....###......#.#.........#..#..
...#...#......#...##....#...#.#
..#...#.#.##.#.................
.....##......#..#.#....#.......
##.......#......#.#..#.#.......
.#.#.#.........#...#.#..#......
#...#.#........#....#.#.....#..
....#.......##....#......##....
.....#..........#......#....#..
#...#....#...#.....#.#.........
...#..##.....##....#.....#.#...
..................#.....##.....
.....#............#............
...#.....#..#........#.#..##...
.......#.#.....................
......#...#.......#..#...#..#..
.#..#...#.....##.....#.#.#....#
....##...#.#............#..#..#
...........#............#..#...
.......#.....#................#
..#......#.#.......#.#.........
.....#..#.#.##.................
.....#..#......................
...#....#...#..#.#..#....#.....
.#............#.....#..........
#.##..#..#.......#......#.....#
.#.........#....#....#.........
...#.#.#........#.#....#...#...
#........#..#..#..........#..#.
.....#..#.....##......##..##.#.
..............#.......#..#..#..
....#........#.##.#...#........
..#.#..#....#........##.....##.
...##.....#...#.......#.#....#.
#.....#..##.##.#...##.......#..
.....#........#.#.#....#.......
.#................#####..#.#...
..........##..#..###....#......
.....#.......#..........#..#...
..#....#....................#..
#.....#..#.....#...##.#.....#.#
...#..##............#.....#....
##.#..#.......##...............
........##...#.#.....#......#..
........#...#..................
#......#................#.#....
...........#...#..#.........#..
...#.##..#.##..................
.....#......###......#..#......
..#.#.....#...#..#.##........#.
....#..........#.#.....#.......
..#..........#..........#.#....
..#.##.......#......#.........."#;

    println!(
        "Day 3 - part 1 result {}",
        count_trees_encountered((3, 1), input)
    );

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    println!(
        "Day 3 - part 2 result {}",
        slopes
            .iter()
            .map(|slope| count_trees_encountered(*slope, input))
            .product::<u32>()
    );

    // Day 4

    let input = r#"hcl:#6b5442 ecl:brn iyr:2019
pid:637485594 hgt:171cm
eyr:2021 byr:1986

eyr:2025 iyr:1938 byr:2014 hcl:#341e13
hgt:66cm
pid:70195175

hcl:#efcc98
iyr:2011 ecl:hzl
eyr:2020 hgt:174cm pid:589700330

hcl:#bba027 eyr:2027 cid:54
ecl:brn pid:153cm
iyr:2028 hgt:173cm
byr:2004

hcl:b45cec
iyr:2011 ecl:oth hgt:185cm eyr:2029 pid:178cm

hgt:185cm iyr:2016 eyr:2029 hcl:#888785 pid:026540921

eyr:2025
hcl:6962f7 byr:2015 ecl:oth iyr:1974
hgt:191cm
pid:2616015

pid:268398556 iyr:2019 ecl:grn
eyr:2027 byr:1951 hcl:#18171d hgt:67cm

eyr:2029 hgt:153cm ecl:brn pid:183179186 byr:2013 hcl:#623a2f
iyr:1957

cid:121 iyr:1922 hcl:752fbc pid:79577560 byr:2025
hgt:61cm eyr:1971

iyr:2016
eyr:2024 hcl:#18171d hgt:184cm
ecl:hzl byr:1992 pid:751161201

eyr:2021 ecl:blu byr:1938 iyr:2016 hcl:#b6652a pid:313406514 hgt:191cm

hcl:#623a2f eyr:2021
ecl:brn
pid:145249653 hgt:167cm iyr:2019 byr:1991

iyr:2022 pid:175cm
byr:2021 eyr:2027 ecl:#f615b1
hgt:172in hcl:#ceb3a1

hgt:173in
ecl:#0cba5e pid:1885981567 iyr:1968
byr:1952
eyr:1942

ecl:oth eyr:2023 hgt:65cm pid:521737908 byr:1971 hcl:z iyr:2017

byr:1936
hcl:#cfa07d
ecl:brn iyr:2011 pid:589047874
eyr:2025

hcl:#fffffd
pid:912552538
cid:159 hgt:160cm iyr:2012
eyr:2023 ecl:hzl
byr:1946

iyr:2015
ecl:amb hgt:72in
cid:59 pid:782818257 hcl:#18171d eyr:2026
byr:1952

hgt:173cm iyr:2018 cid:96 ecl:amb byr:1986 pid:783160698 eyr:2026
hcl:#602927

hcl:#a97842 cid:199 pid:912273414 eyr:2030
hgt:171cm ecl:hzl iyr:2011 byr:1960

ecl:amb hgt:156cm
iyr:2013
hcl:#ceb3a1
cid:116 pid:567057004 byr:1942
eyr:2029

ecl:#cddc40
pid:045090966 cid:254
hgt:75in hcl:#733820 eyr:2026 byr:1956
iyr:2015

pid:156cm
eyr:2040
hgt:176cm ecl:#02e67d hcl:b7c0e6
iyr:1959 cid:129 byr:2022

hgt:160cm byr:1933
ecl:blu eyr:2029 iyr:2012 hcl:#888785 pid:028571975

iyr:2017
hcl:#390f37 hgt:171cm ecl:brn byr:1931 pid:015365720 eyr:2030

iyr:2014 pid:697057757
eyr:2026 hgt:188cm
ecl:gry byr:1926

pid:484310015 hcl:#fffffd hgt:150cm iyr:2018
cid:53 ecl:gry eyr:2021 byr:1957

hgt:156cm
eyr:2026 byr:1963
pid:063272603 ecl:brn iyr:2011
hcl:#888785

byr:1955 pid:310518398 hgt:191cm iyr:2018
ecl:oth eyr:2023 cid:132 hcl:#888785

byr:1938 hcl:#623a2f eyr:2023
iyr:2010
hgt:165cm
pid:170304863
cid:290 ecl:amb

eyr:2026
pid:021468065 hgt:164cm
byr:1996 iyr:2016 hcl:#18171d
ecl:brn

byr:2027 ecl:oth pid:8258823391 hgt:153in hcl:#733820 eyr:1948

byr:2026 ecl:#cd275a iyr:2012 eyr:2036 pid:5917499975

byr:2004
cid:151
hcl:99ecb1
eyr:2033 pid:871137711 iyr:1997
hgt:184cm ecl:oth

byr:2011
hcl:z ecl:#eee1d2 hgt:59cm eyr:1925 iyr:2030 pid:#02ee78

pid:742658992
hcl:#888785
byr:1995
eyr:2024 hgt:162cm iyr:2013 cid:169 ecl:gry

hgt:152cm byr:1946
eyr:2027 iyr:2018
pid:352799678
hcl:#238da0
ecl:amb
cid:71

hcl:#623a2f pid:723616064 eyr:2021
hgt:172cm
byr:1926 iyr:2013
ecl:grn

iyr:2019 hgt:94 byr:2028 eyr:1986
pid:#ee7f00

ecl:amb
eyr:2027 pid:188153423 byr:1957 hcl:#d67ae1
iyr:2011 hgt:183cm

byr:1950 ecl:#e2495d iyr:2010 hgt:166cm eyr:2034 pid:151457075

eyr:1981
byr:2016 iyr:2029 pid:153cm ecl:#55c2a4 hcl:z
hgt:76cm

hgt:184cm ecl:amb eyr:2021
hcl:#623a2f
pid:414607669 iyr:1960 byr:2002

eyr:2027 iyr:2020 hgt:179cm byr:1991
pid:969568248
ecl:blu

hgt:175cm pid:536803427 hcl:#a97842 iyr:2012
ecl:grn byr:1950 eyr:2027

eyr:2028 hgt:60in hcl:#733820 iyr:2018 ecl:oth pid:871909483
byr:1930

hgt:155cm iyr:2020 byr:1960 eyr:2021
pid:515710074 ecl:amb hcl:#341e13

byr:1922 hcl:z iyr:1977 ecl:brn
eyr:2023 hgt:119 pid:865700639

ecl:gry hcl:959fcd pid:#633ac1
byr:2011 hgt:68in
eyr:2020

iyr:1972 hcl:z cid:149 byr:2020
hgt:166in pid:4548657 eyr:1960
ecl:#cc987c

eyr:2023 hcl:#b6652a iyr:2015
hgt:187in pid:547953710 byr:1979 ecl:grn

iyr:2018
pid:508691429 ecl:oth eyr:2025 hgt:187cm cid:270
hcl:#888785 byr:1977

hgt:168cm eyr:2032 byr:2020
ecl:gry iyr:1982
hcl:z pid:648015564

hcl:#fffffd pid:911858643 iyr:2016 ecl:gry eyr:2030 byr:1992 hgt:156cm

pid:241562994 eyr:2026 ecl:grn hgt:164cm
hcl:#c0946f byr:1945 iyr:2015 cid:296

byr:1976 pid:269322775 ecl:hzl
hgt:162cm hcl:#b6652a
eyr:2022 cid:335 iyr:2012

eyr:2028
hgt:106
pid:268626219 hcl:#a97842
iyr:2011
ecl:grn byr:1967

iyr:2016 hcl:#888785 hgt:193cm ecl:oth
pid:034099334 eyr:2027
byr:1945
cid:181

pid:248319556 byr:1987 iyr:2010 cid:122 ecl:utc
hcl:z hgt:190cm eyr:2030

iyr:2019 hcl:#ceb3a1
ecl:hzl
cid:281 hgt:73in byr:1992
eyr:2023

hcl:#fffffd
ecl:blu cid:340 hgt:176cm byr:1980 pid:809878309 iyr:2018

hgt:167cm hcl:#866857 byr:1973 cid:143 eyr:2030 iyr:2012
ecl:hzl pid:168618514

hcl:c97d76 iyr:2016 pid:8439355994 byr:2013 eyr:2036 hgt:71cm
cid:116 ecl:#055b62

hcl:#341e13 pid:961548527 eyr:2027 hgt:192cm byr:1940 iyr:2011 ecl:oth

byr:1935 hgt:189cm ecl:brn iyr:2012
eyr:2028 hcl:#602927

byr:2024
eyr:1939 iyr:2020 hgt:140 pid:889951037
hcl:#b6652a ecl:blu

ecl:amb byr:1942
iyr:2012 pid:161703003 hgt:181cm
eyr:2027 hcl:#602927

hcl:#18171d
iyr:2015 byr:1935
cid:204
ecl:gry
hgt:180cm eyr:2025 pid:988699528

eyr:2025 byr:1985
cid:192
hcl:#866857 hgt:150cm pid:315179208 iyr:2010 ecl:blu

hcl:#341e13 iyr:2013 eyr:2021 cid:62
byr:1928
hgt:168cm pid:862861470 ecl:hzl

pid:099158408
ecl:grn
eyr:2026 iyr:2018 hcl:#b6652a cid:81
hgt:185cm byr:1964

byr:1990 hgt:155cm
ecl:brn
eyr:2023
hcl:#ceb3a1 iyr:2012

ecl:brn
eyr:2026 cid:242 pid:658930205
hgt:176cm byr:1990 iyr:2016 hcl:#d55f68

hcl:#602927 pid:924899781
eyr:2024 byr:1964
iyr:2019
cid:163
hgt:181cm ecl:gry

eyr:2026 ecl:blu pid:8812414708 iyr:2017 hcl:#a97842 hgt:190cm
byr:1970

hgt:152cm
pid:403682313 iyr:2019
hcl:#ceb3a1 ecl:oth
eyr:2021 byr:1957

pid:23799214
byr:2030 hgt:66cm
iyr:2022
hcl:z ecl:#c806fe eyr:2035

eyr:2022 hgt:177cm byr:1967 cid:194
pid:060293594
ecl:brn
iyr:2016
hcl:#cfa07d

hgt:184cm hcl:#6b5442 eyr:2029
ecl:oth iyr:2013 pid:26983291 byr:1965
cid:147

pid:255519852 byr:1975 hgt:192cm
ecl:lzr
iyr:2015 eyr:2030
hcl:#623a2f

iyr:2010
ecl:blu
hcl:#881267 hgt:162cm pid:121130250 byr:1935 cid:57 eyr:2025

hgt:189cm
hcl:#a97842
iyr:2014 eyr:2024
ecl:brn
pid:972960330

hcl:#623a2f eyr:2026 hgt:193cm cid:87 byr:1982 iyr:2020 pid:158154062 ecl:amb

eyr:2025 hgt:191cm
ecl:amb
hcl:#341e13
pid:137048981 iyr:2016 byr:1950

byr:1930 eyr:2029 ecl:hzl hgt:75in
pid:464272185 cid:341
iyr:2012 hcl:#c0946f

ecl:brn
pid:952709301 byr:1926 hcl:#c0946f
eyr:2028
hgt:170cm

pid:578940518 byr:2025 hgt:190in
iyr:2030 cid:52 ecl:amb eyr:2027

ecl:amb hgt:185cm cid:237 iyr:2016 pid:490377510 byr:1950 hcl:#18171d
eyr:2025

iyr:2014 hgt:156in pid:65952131
ecl:blu byr:1938 hcl:#7d3b0c
eyr:2023

ecl:gry iyr:2016 pid:818347623 hcl:#888785 eyr:2030 hgt:174cm

ecl:hzl
hcl:#866857
eyr:2027
pid:213124752 hgt:179cm
byr:1989

pid:024846371 byr:1990 iyr:2018
eyr:2026 hgt:161cm ecl:oth

hcl:z hgt:129 iyr:2016
eyr:2034
pid:#b85e75 byr:2026 ecl:oth

hgt:192cm hcl:#602927 ecl:blu iyr:2011
pid:863613568 byr:1996 eyr:2027

hgt:160cm cid:229 byr:1952
iyr:2019
ecl:#0ae2d6 eyr:2027 pid:719697407 hcl:z

pid:040987502 cid:155 iyr:2012 hgt:173cm
byr:2002
hcl:#fffffd eyr:2023 ecl:hzl

ecl:oth byr:1993 iyr:2019 pid:319157251 hcl:#733820 hgt:70in eyr:2027

hcl:#9d85d4
hgt:192cm pid:570514935
cid:238 eyr:2022 ecl:gry byr:1989
iyr:2016

hgt:162cm
cid:201 iyr:2015 eyr:2023 pid:553794028 byr:1922 ecl:amb hcl:#623a2f

cid:56
eyr:2024 ecl:amb hgt:179cm hcl:#efcc98
pid:665225721
iyr:2012 byr:1963

byr:2026
hcl:#888785
iyr:1972 eyr:1980 cid:323 pid:153cm
hgt:170cm ecl:blu

pid:704204892 ecl:gry
eyr:2023
byr:1920 hgt:168cm iyr:2010 hcl:#3311ec

pid:#7f3caf eyr:2023
hcl:z hgt:146 byr:1990 ecl:amb
iyr:2014 cid:270

hgt:171cm ecl:blu pid:383695713
cid:200 iyr:2010
hcl:#602927 byr:1950 eyr:2024

hgt:178cm byr:1935 hcl:#2da7db
pid:597509269
eyr:2020 iyr:2014
ecl:blu

eyr:2034
ecl:#d4719a
hcl:z hgt:67cm iyr:2023 pid:#268d93 byr:2006

eyr:1939 pid:9942171839
hgt:104
iyr:1945
byr:2011 ecl:#f9bafb hcl:#ceb3a1

byr:1937
iyr:2010 pid:979528684
eyr:2028 hcl:#ceb3a1 ecl:gry hgt:164cm

iyr:2019 eyr:2022 pid:044485658 hcl:#18171d byr:1996 hgt:169cm
ecl:gry

pid:031482456
eyr:2024
iyr:2015
hgt:157cm hcl:#7d3b0c byr:1921
ecl:oth

pid:399398768
ecl:lzr
hcl:z
eyr:1983 hgt:68cm
byr:2024 iyr:2027 cid:127

hgt:186cm eyr:2026 pid:140032921 ecl:amb cid:278
byr:1937 iyr:2015

hgt:172cm
ecl:amb pid:718725983 hcl:#6b5442 eyr:2024
iyr:2013 byr:1974

ecl:amb iyr:2014 cid:216 hcl:#cfa07d
eyr:2022 pid:442275714 hgt:68in byr:1999

hgt:152cm cid:193
iyr:2015 pid:806672342 hcl:#b6652a byr:1927 ecl:oth

hcl:#7d3b0c byr:1925 iyr:2015 hgt:174cm pid:888044223 cid:168 ecl:oth eyr:2029

ecl:gry byr:2009 hgt:156cm
hcl:#888785 pid:263500722 iyr:2015 eyr:2021

cid:103
hcl:#ba8b89 ecl:hzl hgt:169cm
byr:1929 pid:626102979 iyr:2016 eyr:2028

iyr:2016 hgt:188cm cid:133
byr:1926 ecl:hzl eyr:2023 hcl:#602927 pid:678092780

ecl:utc byr:2025 pid:#584dc1 eyr:2037
hgt:151cm iyr:1950 hcl:#cfa07d

ecl:oth hgt:140 eyr:1977 hcl:#6b5442 iyr:1955
byr:1999
pid:868434068

eyr:2029 hcl:#18171d cid:158 iyr:2016 hgt:166cm ecl:hzl
pid:100226690 byr:1942

ecl:#806ce8
cid:153 iyr:2024 byr:1985 hcl:da8a68
pid:#d9e5b0 eyr:2017

eyr:2020 hgt:164cm cid:222 ecl:hzl byr:1945 hcl:#cfa07d
iyr:2011

iyr:2018 hgt:165cm
pid:868536448 eyr:2026 byr:1930
ecl:hzl hcl:#623a2f cid:128

ecl:grn iyr:2012
cid:326 byr:1950 hcl:#efcc98 eyr:2029 hgt:177cm pid:685629972

byr:2004 hgt:168cm
ecl:dne iyr:2020 hcl:z

byr:1964 pid:132604237 ecl:oth hcl:#602927 hgt:188cm
cid:78
iyr:2012 eyr:2025

byr:1945
iyr:2023 ecl:#1a590c hgt:70in
pid:186cm eyr:2031 hcl:z

cid:178
ecl:amb eyr:2024 hgt:162cm
hcl:#18171d iyr:2016
byr:1945 pid:737813370

hcl:#18171d
byr:1949
pid:064917719
hgt:176cm ecl:amb
eyr:2034
iyr:1998

hgt:72in
pid:711343766 hcl:#623a2f
iyr:2010 byr:1977 ecl:amb cid:177 eyr:2023

byr:1933 hgt:66 pid:22149379 eyr:2040
ecl:#92d7a7 hcl:#cfa07d

iyr:2020 byr:1946 eyr:2020 ecl:hzl pid:153cm
hgt:159cm cid:261 hcl:#888785

iyr:2013 byr:1931
ecl:#2ced2e hcl:3c49c1 eyr:1950
hgt:182cm cid:133 pid:#19fc55

hcl:#a9abe6
iyr:2016
eyr:2029 ecl:hzl cid:343 pid:691253232 byr:1952 hgt:187cm

hcl:z
eyr:1964
ecl:#5995e6
byr:2021 hgt:72in pid:2103603035 iyr:1951

iyr:2024 hgt:151in byr:1988 ecl:blu
eyr:1961 cid:117
hcl:z pid:5371118784

hgt:71cm iyr:2021
eyr:2033 ecl:amb
hcl:z cid:202
pid:207141921 byr:1987

ecl:gry byr:1927 eyr:2024
hgt:60in iyr:2014
pid:847799723 cid:285
hcl:#733820

eyr:2022 hcl:#18171d
pid:847063261
byr:1926 ecl:grn
iyr:2011

pid:647225630 iyr:2016 hcl:#a97842 ecl:oth eyr:2025
cid:144 hgt:182cm byr:1983

hgt:150 byr:1924
eyr:2024 hcl:1600da
ecl:brn
cid:168 pid:656253964

hgt:153in pid:644514788 byr:1956 hcl:#866857
iyr:2029
ecl:utc

cid:57 pid:755541617 byr:1961
iyr:2019
ecl:grn
hgt:169cm hcl:#efcc98 eyr:2029

iyr:2005
eyr:2040 hcl:8080a4 byr:2013 pid:145803668

iyr:2029
hcl:z ecl:brn
byr:1948
hgt:76cm pid:186cm eyr:2031

hcl:#888785 ecl:grn byr:1983 cid:268 pid:402246959 iyr:2018
eyr:2020

hgt:175cm eyr:2026 pid:594997236
byr:1991 hcl:#ceb3a1 iyr:2015 ecl:blu

byr:1989
eyr:2027
iyr:2020 hgt:192cm ecl:blu hcl:#cfa07d cid:61 pid:657979353

pid:#a043a3 iyr:2016 byr:1947
eyr:2031 hgt:191cm ecl:xry

eyr:2023 ecl:blu byr:1948 cid:128 hgt:74in
pid:966094274
iyr:2015

iyr:2020 ecl:zzz
eyr:1999 hcl:3cf716 byr:2017 cid:343 pid:60198759
hgt:70cm

hgt:182 pid:80897411 byr:2014 eyr:2033 iyr:1941 ecl:#9c54e8 cid:107
hcl:z

iyr:2015 hcl:#866857 byr:1990 cid:167 pid:588340506 eyr:2030 hgt:168cm ecl:oth

hcl:676aad hgt:151 cid:192 eyr:1930 ecl:oth byr:2012
pid:513365039
iyr:1943

cid:119
ecl:#921980 hgt:70cm
eyr:2024 hcl:4909ee pid:#13fe6c iyr:2022 byr:2014

eyr:2036 hcl:02fdbc hgt:155cm
iyr:1946
pid:508011940 ecl:utc byr:2025

pid:#f74bbe eyr:2028 hcl:#c0946f hgt:171cm ecl:#9077c0
byr:1951 iyr:2010

iyr:2017 hgt:125 hcl:#cfa07d pid:731062033 ecl:brn eyr:2028 cid:255 byr:2020

ecl:xry eyr:2033 byr:1978
iyr:2012 hgt:70cm hcl:z
pid:272848084

ecl:blu hgt:174cm
eyr:2030 byr:1999 hcl:#ceb3a1 iyr:2015
pid:322583115 cid:301

eyr:2007 byr:2007
ecl:dne cid:322 pid:182cm iyr:2013 hgt:156in
hcl:680e8c

hgt:189cm hcl:#18171d
byr:1996 ecl:amb
eyr:2022 iyr:2020 pid:470853813

pid:785152983 iyr:2014 eyr:2028 hcl:#d50ced ecl:hzl byr:1998

ecl:hzl byr:1945 hcl:#7d3b0c cid:164 hgt:187cm pid:414181589 iyr:2018

byr:1936
hgt:183cm ecl:gry pid:376279728 hcl:#7d3b0c
eyr:2023 iyr:2012

byr:2000 hgt:157cm
ecl:hzl
iyr:2020
pid:203994583
eyr:2023 hcl:#866857

eyr:1992 byr:2009
iyr:2029
hcl:dc80b3 hgt:70cm ecl:grn pid:#65c31d

hcl:#7d3b0c
byr:1945
hgt:177cm
iyr:2013 eyr:2028 pid:038116668 cid:74 ecl:blu

pid:700997508 eyr:1970 ecl:zzz hcl:#888785 iyr:2013 byr:1986 cid:219 hgt:76cm

eyr:2025 hgt:161cm
iyr:2015 cid:188
hcl:#fffffd
pid:840085402 ecl:gry byr:1988

pid:96550914 hcl:#481a3b byr:1997 ecl:#a57167
cid:274 hgt:174cm

hcl:#b6652a
ecl:brn eyr:2029
hgt:157cm iyr:2011 pid:910022061
byr:1947 cid:273

pid:010289131 eyr:2026
byr:1930
hcl:#b6652a ecl:grn
cid:220 hgt:187cm iyr:2013

hcl:#6b5442 ecl:grt hgt:120
pid:454504291 eyr:1933 byr:2025 iyr:1930

iyr:2016
hgt:180cm ecl:amb eyr:2028 cid:237
pid:334803890 byr:1953 hcl:#18171d

eyr:2020 byr:2002 hcl:#c54f21
iyr:2019 ecl:blu hgt:180cm cid:138

byr:1933
iyr:2020
ecl:brn hgt:185cm
hcl:#c0946f
eyr:2020 pid:050791974

byr:1933 ecl:brn hgt:186cm
pid:643899463 eyr:2030 iyr:2019
hcl:#866857

iyr:2018 byr:1935 ecl:oth
eyr:2029
pid:732801213 hcl:#6b5442 hgt:169cm

eyr:2020
hcl:z byr:1996
ecl:#4102ee
pid:890541531 hgt:193cm iyr:2014

pid:618379341 ecl:gry hcl:#18171d byr:1991 eyr:2025 hgt:154cm iyr:2019

iyr:2013
pid:912066964 ecl:grn eyr:2040 hgt:192cm byr:1974
hcl:#18171d

eyr:2025 cid:167 hgt:192cm
pid:678328147 ecl:gry
hcl:#18171d iyr:2017

iyr:2011 byr:2021 hgt:189cm ecl:gmt hcl:z eyr:2035 pid:278839955

eyr:2030 hcl:#efcc98
ecl:blu iyr:2011
pid:536958012 hgt:192cm byr:2002

pid:#1306f2 byr:1976
hcl:#790688 hgt:158cm ecl:grn eyr:2024 iyr:2019

eyr:2030 hcl:#866857
cid:50 ecl:oth pid:421235317
iyr:2014 hgt:60in

iyr:2020 byr:1971 cid:124
pid:319896110 ecl:oth hcl:#fffffd

cid:143
eyr:2021 hgt:190cm pid:366021385 hcl:#18171d ecl:amb byr:1934
iyr:2016

hgt:169cm hcl:#602927 pid:177cm
eyr:2022 byr:2020 ecl:#dd96f4 iyr:2014

eyr:2020 hgt:173cm pid:591592395 ecl:oth byr:1966
hcl:#c0946f iyr:2020

pid:282088832 ecl:gmt
hgt:167in byr:2016 hcl:z
iyr:2018

iyr:2016
hgt:62in hcl:#c0946f
pid:306204399 eyr:2020 ecl:brn
byr:1999

eyr:1947 byr:1984 pid:595671246 hcl:3d50e7 ecl:xry iyr:1947

hgt:187cm
eyr:2024 pid:477011496
byr:1971
hcl:#733820
iyr:2010
ecl:brn cid:165

byr:2023
pid:173cm
hgt:193in eyr:2019 cid:102 ecl:grt hcl:#c0946f

pid:195062251
iyr:2027
cid:138 byr:2021 ecl:brn eyr:2025 hgt:60in

hgt:71cm hcl:z
ecl:utc
eyr:2021 iyr:1925 pid:5469028726 byr:2017

hcl:#b6652a hgt:168cm
byr:1960 ecl:grn
pid:653140418
iyr:2014 eyr:2023

pid:#afa892 cid:190 hcl:z
hgt:189cm
eyr:2020 ecl:gry
byr:2003
iyr:1956

hcl:e4cddf cid:185 pid:189cm hgt:175cm
byr:2016 iyr:2010 ecl:#fa945d eyr:1947

cid:176
hcl:7752f8 eyr:2039 byr:2019 ecl:hzl iyr:2029 hgt:185cm pid:636534364

cid:170 ecl:gmt hcl:ef5177 byr:2021
eyr:1993
hgt:71cm pid:2136295 iyr:2013

byr:2028 pid:156cm hcl:d74b86 cid:238
hgt:89
iyr:1957 eyr:1937

eyr:2030 byr:1932 hcl:#c0946f cid:349
hgt:177cm
ecl:grn iyr:2016

hcl:z byr:2003
ecl:#9b98b2 hgt:81 pid:13338103 eyr:2040

iyr:2018 pid:801432704 hgt:73in byr:1964 cid:298 hcl:#fffffd ecl:amb eyr:2030

cid:272
iyr:2019 pid:369160624 byr:1929 hgt:184cm eyr:2025 hcl:#ceb3a1 ecl:blu

hcl:#7d3b0c pid:525287934
byr:1998 eyr:2027
iyr:2017 hgt:168cm ecl:gry

byr:1975 eyr:2027
ecl:brn cid:125 hcl:4e319d
hgt:172cm pid:308046532 iyr:2017

hcl:b889c0 pid:6699675552 byr:2019 iyr:1968
ecl:gmt
eyr:2003
hgt:180in

byr:2025
ecl:zzz hgt:162in hcl:z iyr:2002 pid:#87dca4 eyr:1951

eyr:2022
pid:549517742 ecl:hzl
iyr:2026
byr:2029 hgt:153cm hcl:2993de

eyr:2024
pid:755674604 iyr:2018 hcl:#c0946f
ecl:gry byr:1966
hgt:188cm

pid:665375893 iyr:2017
byr:1997
eyr:2029 hgt:173cm ecl:gry

hcl:#6b5442 hgt:74cm ecl:#0dc7f6
pid:451038742 eyr:1982 byr:1939
iyr:1932

hcl:#18171d
byr:1980 ecl:gry
iyr:2019 hgt:167cm
pid:326267989 eyr:2028

cid:226 hgt:177cm ecl:hzl hcl:#a97842 eyr:2025
iyr:2013
byr:1949 pid:292166795

ecl:oth pid:962521763
iyr:2013 cid:71 eyr:2022 hgt:193cm hcl:#18171d byr:1969

ecl:grn iyr:2028 eyr:2024
hgt:189cm hcl:z byr:1940 pid:032392876

iyr:2012 hgt:191cm cid:339 ecl:oth eyr:2028 pid:392810631 hcl:#b6652a byr:1959

iyr:2011 byr:1975
eyr:2027
hcl:#18171d
hgt:176cm
ecl:gry pid:290432747

cid:180 ecl:brn pid:210871734 eyr:2027
byr:1946 hcl:z hgt:185cm iyr:2011

byr:1924 ecl:grt
eyr:2028 hgt:187cm pid:#608f4f

eyr:2022 ecl:#a05063 byr:1926 hcl:#7d3b0c pid:3292990618 hgt:183in iyr:2021

ecl:#a8b66c
iyr:1942 eyr:1960 hgt:60cm byr:2027 pid:#3b6f3f hcl:9fae56

hgt:183cm
ecl:oth hcl:#c0946f pid:816986054 eyr:2020 iyr:2014 byr:1935

hgt:174 byr:2008
iyr:2029 hcl:9259e7 pid:85036214 ecl:gmt

cid:85
pid:032040868
byr:2001 eyr:2027 hcl:#c0946f ecl:grn iyr:2020
hgt:173cm

hcl:#6b5442
cid:308
ecl:grt iyr:1939 byr:2009
pid:9365125584 eyr:2031 hgt:67cm

hgt:154cm
byr:1936
eyr:2030 hcl:491c70 pid:391887956 iyr:2029 ecl:blu

hcl:#866857
hgt:161cm cid:76 pid:921202500 eyr:2021 ecl:brn byr:1968

iyr:2024 ecl:dne hcl:z pid:8054447805 hgt:154 eyr:2035 byr:2024

hcl:#0a524b pid:667928918
eyr:2025
cid:245 ecl:brn byr:1973 hgt:179cm

ecl:gry hgt:68in pid:322837855 eyr:2023
cid:323 byr:1944
iyr:2012

byr:1940
hgt:178cm ecl:hzl hcl:#c0946f iyr:2030
eyr:2020 pid:788531859

cid:253 iyr:2012 hgt:163cm
pid:554364568 eyr:2025 byr:1976 ecl:grn hcl:#888785

hcl:#efcc98 iyr:2015 ecl:gry eyr:2029 pid:273847553 cid:274
hgt:68in byr:1933

hgt:165cm
pid:209462386 eyr:2024
byr:1969 hcl:#733820 ecl:grn
iyr:2020

byr:1975 hgt:187cm eyr:2027 iyr:2018 hcl:#c0946f ecl:hzl pid:141969110

hcl:z pid:534439483 iyr:2022 ecl:grt eyr:2036 hgt:164in cid:324
byr:2025

hcl:#74ca66
iyr:2011
pid:253012158
hgt:188cm
cid:246 ecl:oth eyr:2023

byr:2020 pid:56939101 hcl:9f5f65
eyr:1949
iyr:2021 hgt:155in

iyr:2020 hgt:174cm cid:304
byr:1944
eyr:2028 hcl:#733820

hcl:#866857 ecl:gry eyr:2030 iyr:2014 hgt:63in byr:1997
pid:371522079

ecl:amb
byr:1955 iyr:2013
hcl:#888785 cid:265 eyr:2026 hgt:190cm pid:311393763

eyr:2026 iyr:2019
pid:721355771 byr:1947
hcl:#733820
hgt:71in ecl:gry

cid:94 eyr:2024 byr:1938 pid:336868233 ecl:hzl
iyr:2012
hgt:177cm hcl:#7d3b0c

ecl:brn iyr:2010
eyr:2027
pid:379769214
cid:111 byr:1960 hcl:#cfa07d hgt:169cm

hgt:179cm
hcl:3f59a6 eyr:2036 byr:2025 ecl:oth pid:217404607
iyr:2018

ecl:amb pid:820370865 hgt:170cm iyr:2012 byr:1967 hcl:#efcc98 cid:309 eyr:2025

byr:1940
pid:008495978 ecl:gry hgt:159cm hcl:#602927 eyr:2024

hgt:186cm
byr:1971 pid:556900517 cid:334 hcl:#efcc98 ecl:brn
iyr:2014

iyr:2020 byr:1928
cid:200 hgt:193cm
ecl:grn hcl:#7d3b0c

cid:233
hcl:a3046e pid:626863952 ecl:lzr iyr:2029 eyr:2024 byr:2000 hgt:193cm

cid:244
hcl:#866857 ecl:amb byr:1931
eyr:1928 pid:557376401 hgt:182cm iyr:2013"#;

    let mut passports = Vec::new();
    let mut passport = String::new();

    for line in input.lines() {
        let line = line.trim();

        if line.is_empty() {
            passports.push(passport);
            passport = String::new();
            continue;
        }
        passport.push_str(line);
        passport.push(' ');
    }

    passports.push(passport);

    let mut counter = 0;

    for passport in &passports {
        if is_valid_passport(&passport) {
            counter += 1;
        }
    }

    println!("Day 4 - part 1 result {}", counter);

    let mut counter = 0;

    for passport in passports {
        if is_valid_passport_with_data_validation(&passport) {
            counter += 1;
        }
    }

    println!("Day 4 - part 2 result {}", counter);

    let input = r#"FFFBFFFRRL
FFFBBBFLLL
BFFFBFBRLR
BFBFFFBRLL
FFFBFBBRLR
FBBBBFBLRR
BFBBBFFLRR
FBFFBBFLRL
BFBBBBFRRL
FBBFBFFRRL
BBFFFBBLRR
BFFFFFBRLR
FBFFBFBLLR
FBBFFFFLRR
FFFFBBFRLR
BBBFBFBLRL
FFFBFFBRLL
BBBFBBBRLL
FBFFFBBRLR
BFBBBBBLRL
BFBFBBFRRR
BFFFBBBLRL
FBBFFBBRLR
FBFBFFFRLR
FBBFFBFLRR
BFFFFFFLLR
FBBFBBFRLR
BFBFBFFLLL
FFFBBBBRLL
FBFFFFBLLR
FFFBBBBLRR
FFFFBFBRRL
BBFBBBBLRL
BBBBFFBLRR
BFFBBFBRLR
BBBFFFFRLR
BBBBFFFRLR
FBBFFBBRRL
FFFFBFBRLL
FBFFFBFRLL
FBBBFBFLLR
FFFFBBFRLL
BBFFFBFLRL
FFBFBFBRLR
FBFFBBBLRR
FFFBFFBLRL
FFBFBBFLLR
FFBFFFBLLR
FBFBBBBRLL
FBBBBFBRLR
FBFFBBFRLL
FFBFFBFLLR
BBBFFBBLRL
FFFBFFFLLR
BBFBBBFRLR
BBFFFFBRRL
BFBFFFBLRL
BBFFBFBRRL
BBFBBFBRRR
FFBBBFBRRL
FFFBFBBRRL
BBFBBBFLRL
FBBFBBBRRR
FFBFFBFRLL
BBBFBFFLRL
FBFFFFFLLR
FFFBBFBLRL
BBFFBFBRLL
BBFBFFFRRL
FBFBBFFRLL
FBFBBFFLLR
BFBFFBFRLR
FBFBFFBLLL
BFFFBBBRLR
BBBFBFBRRL
BFFBFFBLRL
FBFFBFBLLL
FBBBFBFRLL
BFBFBFFLLR
BFFFFBFRLL
BFFBBFBLLR
FFBBBBFRRL
BFBBFBBRRR
FBFBFFFRRL
FFBFFFBRLR
BBBFBBFLRL
BFFBFFBLRR
FFBFBFBLRL
FFBFBFFLLR
BFBFBBBRRL
FFBFFFFLRR
BFFBBBBRLL
FFFBFFBRRR
BBBBFFBLRL
FFBFBFFLRL
BFFBFFFLRL
BBBFBFFRLR
BBBFBFFRLL
BBFBBBBRLL
FFFBBBFLLR
BFFFBBBRRR
BBFFFFFLLL
BBFBFBBLRR
BFBBFFBRRL
BFBBFFFRRR
BFBBBBFRLR
FBFFFFBRLR
FFBBFBFLRR
BFFBFFBRLL
FBFFFBBRRL
BBFFBBFRRR
FBFBFBFLRL
FBFBBFFRRL
BFFBFBFRLL
BBBFFBFLLR
FFBFBBBRRR
FBFFBBFLLL
FFBFFBBLRR
BBBFBBFRLL
FFBBFFFRRL
FFBFFBFLLL
FFFBBBBLRL
FFFBFBFRRR
BBFFFFFLRR
FBFBFBBLRR
BBFFFFBLRR
BBFFFBFLLR
FBFBFBFLLR
FFFFBBBLLL
FBBBFFFLRL
BFBBFFBLLR
FFBBFFFLRR
FBFBBFBLLR
FFFBBBFRRR
FBBFBFBRRR
FBBBBFFLRL
FFBFFBBRRL
BBFFFFFRRR
BFBBFFBLLL
FFBBFBFRRR
BBFFBBFRLL
BBBBFFBRRR
BBFBFBBRLR
BFBFFFFLRR
FFFBBBFRLR
BFBBBFBLRR
FBFBBBFLLR
FFBBFFFLRL
FBFBFFFLLL
BBBFFBBRRL
BBBBFFFLRL
BFBFFBBRLR
FBFBFBBLLL
BFFBBBBLLR
BBBFBBBRLR
BBFFFBBLLL
FBBBFFFLLR
BBFBBFFRLL
BFBBFFFLLL
FFFFBBBRLL
BFFBBBFRLL
BFBBBBFLRR
BFBBFBBRLL
BFFBBBFRRL
BFBFBBFRRL
FBBFFBFRRL
FFBFFBBLRL
FBBFFBBLRR
FBFBFBBLRL
FBFBBFBLRL
FBBBFFBRRL
BFFFFFBRRR
BBBFBBBLRL
FBBFFBFLLR
BFFBBBBRRR
FFFBFBBRRR
FFFBBFBLLR
FBBFFFFLRL
BBBFBFFLRR
BBBBFFBRLL
BFBBFBFLRR
FBFBFBBLLR
BFBFFBFRLL
BBBFFFBLLL
FBFFFBBLRL
BFBFFBBRRR
FBBFBFBRRL
FFFBFBFLRR
BBBFFBFLRL
BFBBFBBLRL
FBFFBBFRLR
BFFFFBFLRL
BFBBBFBLLR
BFBFBFFLRL
BFBBBBBLLL
FFFBBFBRLR
BBBBFBFLLL
BBFFFFFRLL
BBFBFBFRRL
FFFBBBBRRL
FFFBFBFRRL
BBBBFFBRRL
BFFBFFFLLL
BFBBBFBLRL
FFFBBFBRRL
FBFBBFFLRR
FFFBFFFRLL
BBFFBBBRRL
FBFFFFFLRR
BBBBFFFRRL
FFFBFFBLLR
BBFBBBFRRL
FFFBFBFLLL
FBBFFBFRLL
BFFFFBBRRL
BFBFBFBLRR
FBFBFBFLRR
BBFBBFBLLR
BBBFFBBLRR
FBBFFBFRLR
BBBFFFBLRL
FFBFFBFRLR
FFFBBFFLLR
FFBFFFBRLL
BFBFFFBRRL
FBBBFBFLRR
BBFFFBBLRL
BFFBBFBLRR
BFFFBBFLLR
BFBFBBBLLL
BFBFBBBRRR
FFBFBFFRLL
FFBBBBBRRL
FFBFFFFRRR
FFBBBBFRLR
FBFFFFBRRR
BFFBFFFLRR
BBFBFFBLRL
FBBBBBFRRR
BBFFBBFLLL
FFFBBBFLRL
FFBBFBBLLR
FFBFFFBRRL
FBBBFBBLRL
BFFBFBFLRL
FBBFFFFRLR
FFFBBFBRRR
BFFBFBBLRL
BFBBFBBLLR
FBBFBFFLLR
FFFFBFBRLR
BFFFFFBRLL
FBBBBBBLRL
BFFBBBFRLR
BBFBFBFRRR
BBFBFFFRLR
BBFBFFBLRR
FFFBFFFRLR
FBFBBBBRRL
BBBFFFBRLR
FFFBFFBLRR
FBFFFBFRRR
FFFFBBBRLR
BBBFFFBRLL
FFBBBFFLRR
BFBBFFBRRR
FBFBBBFLRL
BFBFFBFRRL
BBFBFBFRLR
BFBBBBBRLR
FFFFBBFLRR
FBFBBFBLRR
FFBFBFBLLL
FFFBBBBRRR
BBFFBBBRLR
BFFFBFBLLR
BFFFBFFLLR
FFFFBBFLRL
FBBBFBBRLL
BFBBFBBLRR
BBFBBFFRLR
BFBBFFFLLR
BBBFFFBLRR
BFFBBFFLRR
BBBFFBFRRL
BBFFBFFLLL
BFBFFFFRRR
FBFBBFFLRL
FBBBBBFLRL
FFBBFBFLRL
BFBBFBFLRL
FBBFFBBRLL
BBBFFFFLRL
BFFFBBBLLL
FFBBBFBRLR
FFBFBBFRRL
FBFFBBBRLL
BBFBBFBRLL
BFFFFFFLRR
BFFFFBBRRR
FFFFBBFLLR
BFFFBBFLLL
BBFFFFBRRR
BFFBBBBLLL
BFFBFFBLLR
FFFFBBFRRL
FFBFBFFRRR
BFBFFFFLLL
BBBFFFBRRR
FFBBBFBRRR
FBBBFFBRLR
BFFBBBFLLL
FBFBBFFRLR
FBFFBBFRRR
FBFBFBFRLL
FBBBFFFLLL
BFFBFFBRRL
FBFFBBFLRR
BFBBBBBLRR
FBBBBBFRLR
FFFFBBBLRL
BBBFBFBLLL
BBFFBBBLRL
BFFBBFFRLL
BBBFFFFLLL
BBBBFFFLLR
BBFBBFFLRR
BFBFBFBLRL
BBFFFBBRRR
BBBFFFFLLR
FBBBFFBRLL
BFBBFFFRRL
FFBBBFBLRR
BFFBBBFLLR
FFBFBBBRLL
FBBBBBBLRR
BFFBBBBRRL
BFBBBFBLLL
FBBFBFBLRR
BFFFBFBRRR
FBBFFFBRRL
FFFBFBBLLR
BBBFFFFRRL
BFBBFFBRLR
FBBBFBBLRR
FBBFFFBLLR
FBBFBBFRLL
BBFBBBFLLR
BBFFFBFRRL
FBBBBFFRLR
FFBFBFBRRL
FFFBFFFRRR
BFFBFBFLLR
BFBFFFFRLL
BFFFFFBLLL
BFBBFFFLRR
FFBFFFBRRR
BBFFBBBRLL
FBFFFFFRLL
FFFBFFBRLR
FBFFBFFRLL
FFFBFFFLRL
BBFBFFBLLL
BBFFFFFLLR
FBBBFFFRLR
BFBFFBFLRR
FFFBBFBLRR
FBFFFBFLLR
BFFFBBFLRL
BFBBBBFRLL
FFBBFFFRLL
BBFBFBFLLL
FBFFBFFRRL
FBFBFBBRRL
FBFFFFFLLL
BBBFFFFRRR
BFBBBFFLLL
FBBFFFBLLL
FFBBBFFRLL
BFBBFBFRRR
FFBBBBFLRL
BFBBFFBRLL
FFBBBFFRRL
FBFBBBBLRL
FFBFBFBLRR
FBFFBFBLRR
FFBFFFBLRL
BBFFBFBLLL
FBFBBBFRLL
FFBFFBFLRL
FBFBFBBRLR
BFFBBFBRRL
BFFFFFBLRL
FFBFBBBLRR
FBBBFFBLLL
BFBFBFFRRR
BBBFBFBLLR
FBFBBFFRRR
BFFFFFBRRL
FFBFFBBLLL
FBFFBBFRRL
FBBBBBBRLL
FFBBFBBRLL
BBFBFBBLLR
BBBFBFBLRR
BBFFBBBLLL
FBFFFFBLLL
BFBBFBFLLR
FBFFBFBRLL
BFFBFFFRRR
FFBBBFFRLR
FBFFBBFLLR
BBBFBFBRLL
FBBFFBFLLL
BFFBBBFLRL
BBBBFFFRRR
BBBFBBFRRR
FFBFBFBLLR
BFFBBFFRRL
BBFFFFFRLR
BBFBBBBRRR
BBFFBFBLRL
FBFFFFBLRR
FFBFBBFLRL
BFBFFBBLLL
BBBFFBFLLL
BBFBFFFLRR
BFBBBBBLLR
FFBFFFBLLL
FBFFFBBLLR
BFBFFBBLRR
FFBBBFFLRL
FBBFBFFLRR
FBBFBFFRLL
FBBFBFBRLR
FBBFFBBLLL
BBFFBFFLRL
FBFBBBFRRR
FFBFFBBRRR
FBBBBBFRLL
BBFFBFBLRR
BBBFFBBRRR
FFBBBBBRRR
BBFFBBBLLR
FBBFBFBLLL
BBBFBFFLLL
BBFFBBBLRR
BFBFBFFRLR
BBBFBBBLLL
FBBFBFBLLR
FBBBBFFLLR
BFFFBBFRLR
FFFFBFBRRR
BFBFBFBLLR
FFBFFBFRRL
BFBBFFBLRL
BFBBBFFLLR
BBFFBFFLRR
FBBBBFBRRR
FBFFFBBLRR
BBFBBBBLLL
FBFBBBBRLR
FFFBBFFLRR
FFBFFBBLLR
FBBBBBBRRR
FFFFBBFLLL
BFFBFFBRRR
FFBBFFBRLR
BBFFFFFLRL
BBBFBBBLRR
BFBFBFFLRR
BFFFBBFRLL
BFBBFBFLLL
BFBFFBFLLR
FFFFBBBLRR
BBFBFFBRLL
FFBBFBFRLR
BBBFBBBRRL
BFBFBFBRRR
BFFFFFFRLR
FBBBBFFLRR
FBFBFFBLRL
FFBBBFFLLR
FBBFFFFRLL
FBFBFBFRRL
BFFBFFFRLR
BFFFBFFRLL
BBFBFBBRLL
FFBBFBBLRR
FBFBBFFLLL
FBFBFFFLLR
BFFBBFFLLR
FFBBFFBRLL
FBBBFFBLRL
BBFBBBBLRR
FFBFBBFRRR
FFBFBBBLLR
FFBBFBFRRL
BBFFFBBRLR
FFBBFFBLLL
BFBFBBBLRR
FFBFBFBRLL
BFBBBFFRRL
BFBFBFBRLR
FFBFFBBRLL
FFBBBBFRLL
FBBBFBFRRL
BBFBFBFRLL
BFFBFFFRRL
BFFBBBFRRR
BBFBFFFLRL
FBBBFBBLLR
BFBFBBBRLR
BBBFBFBRRR
BFFBFBFRRL
FFBBFFFLLR
FFBFFFFRLR
FBFBFFBRLR
BFBFFFBRLR
FFBFFBFLRR
BBFBFBBLRL
BBFBFFFLLR
BFBFFBBLRL
FFBBFFBLRL
BFFBBBBLRR
BBFBBFFRRL
FBBFBFFLLL
BFBFBFFRRL
FFFBFFBLLL
FBBFFFBRLR
BFBBFBBRLR
BFFBBFFLRL
BFFFFFFLLL
FBFBBBFRLR
BFBFBFFRLL
BBFFBFFRRR
FBBBFBFLRL
FFFFBFBLLR
FBFBFBBRLL
FBBFFBFRRR
BBFBFFBLLR
FBFBBFBRRL
BFBBBFFLRL
BBBBFBFRLL
BFFFFFBLRR
BBBBFFFLLL
BFFBFBBLLR
FFFBBFBRLL
FBBBFBFLLL
BFFFBFFRLR
FBFFBFFRLR
BBFFFFBLRL
BFFBBFBRLL
FFFBBFFRRR
FFBBFBFLLL
BBBFFBFLRR
BFBBBFFRLL
BFBFFFFRRL
FFBBFBFLLR
BBFFBBFLRR
FFBBFBBRRR
FBBBBBBLLR
FFFBBBBLLR
BBFFFFBRLR
BFBFFBFLRL
FBFFFBBRLL
BBBFBBFLLR
BFFBBFBLLL
BFBBFBFRLL
BBFBFBFLLR
FFBBBFFLLL
FBBFBBBRLR
FBFFBFFLLL
BFBBBBBRRR
FFBFBBBRRL
BFFBFBBLLL
BBFFBFFLLR
FBFBFFBRRR
BBFBFFBRRL
BBBBFBFLLR
BFFFFFBLLR
FFFBFBBLRL
FBBFBBFLRL
BBBFFFBRRL
BFBBBFBRLL
FBFFFBFLRL
BBBFFBFRLL
BFBFFFFRLR
BFFFFFFLRL
BBFFFBBLLR
BBFBBFBLRR
BFBFBBBLRL
FFFFBBBRRL
FBBFBFBLRL
FBFBBBBLRR
FFBBBBFRRR
BBFFBFBRRR
BBFFFBFRRR
BFBBBFBRRR
BBFFBBFLLR
FBFFBBBLRL
BFBBFBFRRL
FBFBFBFRLR
BFBFFFBLLR
FBFFBFBRLR
BFBFFBBRLL
BFBFBFBRLL
FBBBFBFRRR
FBBBFBBRRL
FBFFFBFRLR
FBFBBBFRRL
BFBFFFBLRR
BBBBFBFLRL
BBBFBFFRRL
BFFFFFFRRL
BFBBBFBRRL
FBBBBFBLLL
BBFBBBFLLL
BFFFBBBRLL
FFBFBBFRLL
BBFBBFFLLL
BFFBFBFLLL
FFBBBBBLLL
FBBFFFFLLL
BBBFFFFLRR
BFFFFBFLRR
BFFBFBBLRR
BFFBFBBRLR
FBBBFFBLLR
BFFBFBFRLR
BBFBBBBRLR
BBFBBFBRLR
FFBFBBBRLR
BFFFBBFRRL
BBBFFBFRRR
BFBBFFFRLL
FFFBBFFRRL
FFFFBFBLLL
FFFFBFBLRL
BBBFFBBRLL
FFFBFBFRLR
BFFFBBFLRR
BFFFFBFLLL
FFBBFFBRRL
FBFFFBBRRR
FFBFBFFRRL
BFBBFFBLRR
BBFFBBFRRL
FFBBBBBLRR
BFBBFFFRLR
BFFFFBFLLR
FBFFBFFLLR
BFFFFBBRLL
BBFFBFBRLR
FBFBFFFLRL
FBFBFFFRLL
FBFFFFBRRL
BFFFBFBLRR
BFBFFBFRRR
FFBBFFFLLL
BFBFBBFLLR
FFFBFBBLRR
FBBBBBBLLL
BBBFBBBRRR
FBFFBFBRRR
FBBFBBBLLR
FBBFFBFLRL
BBFBBFBLLL
BBFBBBBRRL
FBFBBBBLLL
FBBBBFFRRR
FFBBFFBLRR
FBFFBBBRRR
FFBBFBBRLR
FFFBFFBRRL
BBBBFFBLLR
BBFBBFFLLR
BBFFBFFRLL
FBFFBBBRLR
FFBBBBFLRR
FBBFFBBLLR
FBBBBFBLLR
FBFBFFBLRR
BFFFBFFRRL
FBBBBFFRRL
FFBBFFBLLR
BBBFFBFRLR
BFBBBBFLLL
BBFBBBFRRR
FFFBFFFLRR
BBFFBBBRRR
FBBFBBFRRR
BFFBBFFRRR
FFBBFFFRRR
BFFBFBBRLL
FBBBFBBLLL
FBFBFFBRLL
FBBFBBBLLL
BFBFFFBLLL
FFFBFBFLLR
BBFFBFFRRL
BFBBBBBRRL
FBFFFFFLRL
BFFFBFFLLL
FBBFFFBRRR
FBBFBBFLRR
BFFFBBBRRL
BFBFFFFLRL
FBFFBFBLRL
BBBFFFBLLR
FBBFFBBLRL
FFFBFBFLRL
BBFFBFFRLR
FBFFFFBRLL
BBFBBFBLRL
BFFFBFFLRL
FBBFFFFLLR
BFFBBFFRLR
FFBFFFFLRL
FBFFFFBLRL
FBBBBBBRLR
BBFFFBBRLL
BFFFBBBLLR
BFFBBBBRLR
FBBBBFFLLL
FFBFBFFRLR
BFFFFBBLLL
FBFFFBFRRL
BFBFBBFRLL
FFBBBFBRLL
FBBFBBBLRR
FBBFFFBLRR
BFFFFFFRLL
FFBBBBBLLR
BBBBFBFLRR
BFBFBBBRLL
BFBBBBFLLR
FFFBFFFLLL
BFFBFBFRRR
BBBBFFFLRR
FBBFFFFRRR
BFFFBBFRRR
FBBFBFFRRR
BFBBFBBLLL
FFBBBBBRLR
BBBFFBBLLR
BBBFBFFRRR
BBFFFFBLLL
BFFBFFFLLR
FFBFBBFLRR
FFBBFFFRLR
FBFBBFBLLL
FBBBFBBRRR
FBFFBFBRRL
BFFFBFBLLL
FFFFBBBLLR
BFFBBFBLRL
BFFBBBBLRL
FBFFFFFRLR
FBFFBFFLRR
BFFFFBBLRR
FBBBFBFRLR
BBBFBBFLLL
FFBFFBBRLR
BBFBFBFLRR
FBFBBFBRLL
BFFBFBFLRR
FBBBBBFRRL
FBBBBBBRRL
BBFBFFFLLL
FFBBBFFRRR
BFBBBFBRLR
BFBFFFFLLR
BBBFFFFRLL
FBBFFFBRLL
BFFFFFFRRR
FBFFFFFRRR
FBFFBBBLLL
BFBFBBFLRR
BBFBBBBLLR
FBFBBBFLRR
FFFBBFFRLL
BFBFBBBLLR
FFBBFBBRRL
FBFBBBBLLR
BBFBFBBLLL
FBBFBBBRRL
FBBBBFFRLL
FBFBFBBRRR
BFBFBBFRLR
BFBBFBFRLR
FBFFBBBLLR
BFBFBFBRRL
BFFBFFFRLL
BFFFFBFRLR
FBBBFBBRLR
FFBFFFFRLL
FBBFBBFLLR
BFBFFBBRRL
FFBFBBBLRL
BBBFBFFLLR
FBFBFFFRRR
FFFBBBFRLL
BBBFBBBLLR
FBBBFFFRLL
FFBFBFFLLL
FFBBBBBLRL
BFBBBFFRLR
FFBBBBFLLL
FBFFFBFLLL
FFFBBBFLRR
BBFBBBFLRR
FBBFBBFRRL
FBBBBBFLRR
BBFFBBFLRL
FFBFFBFRRR
FBBBFFFRRL
FFBBBFBLRL
BBFFFBBRRL
FBBBFFFRRR
FFFFBBBRRR
FBBBBBFLLR
FBBBBFBRLL
FBFBBBBRRR
FFBFFFFLLR
BBBBFFBLLL
FBFFFFFRRL
BBFFFFBLLR
FFBBFBBLRL
FBFFFBFLRR
BFBFBBFLRL
BBBFBBFRRL
FFFBBFFLRL
FBBFBFFLRL
FBBFBFFRLR
BBFBFBBRRR
BFBFBBFLLL
BBFBBBFRLL
FBBFBBBLRL
BFFFFBFRRL
FBFBFBFRRR
FBFBBBFLLL
BFFFFBBLLR
BBFBBFFRRR
FFBFBFFLRR
BBFFFBFRLR
BBBFFBBRLR
FBFFBFFRRR
FFBFBBFLLL
BFFBBBFLRR
FBFBFBFLLL
BFFFFBBRLR
FFFBBFBLLL
BBBFBFBRLR
BFBFFBFLLL
FFFBBBBLLL
FFBFBFBRRR
FBFBBFBRRR
FBBFBBFLLL
BBFBFBFLRL
BFBBBBBRLL
FFBBFFBRRR
FFFFBBFRRR
BFBBBFFRRR
FFFBFBFRLL
FFBFFFFRRL
FFFBBBBRLR
FBBBFFBRRR
BBFFFBFLRR
FBFFFBBLLL
FBBBBBFLLL
BBFBFFFRLL
BFFFBBBLRR
FFBFBBBLLL
BBFFFBFRLL
FFBBBBBRLL
FBFFBFFLRL
FFFFBFBLRR
FBFBFFBRRL
BFFFFBFRRR
FBBBBFBLRL
BBBFBBFRLR
FFBBBBFLLR
FBFBBFBRLR
FBBFBBBRLL
FBBBFFBLRR
FBBFBFBRLL
FFBBFBFRLL
FFFBFBBRLL
BBFFBFBLLR
FBFBFFBLLR
BBFBFFBRLR
BFFBBFBRRR
BFFFBFBLRL
BFBFBFBLLL
BBBBFFFRLL
FBBFFBBRRR
BBFBFFBRRR
BBFBFBBRRL
BFBBBBFRRR
BFFFFBBLRL
BBBFFBBLLL
BFBBFFFLRL
BFFFBFFRRR
BFFBFFBRLR
FBBFFFBLRL
BFBBBBFLRL
FBFBFFFLRR
FFFBBBFRRL
BFFBFFBLLL
BFBBFBBRRL
BFFFBFBRRL
BFBFFBBLLR
FFBBBFBLLR
FFBBFBBLLL
FFFBFBBLLL
BBFBBFBRRL
BFFBBFFLLL
BFFFBFFLRR
BFBFFFBRRR
BBBBFFBRLR
FBFFBBBRRL
BBBFBBFLRR
BBFFFFBRLL
BBFBFFFRRR
FFBBBFBLLL
FBBBBFBRRL
BBFFFFFRRL
FFBFFFBLRR
BFFBFBBRRL
FFBFBBFRLR
FBBBFFFLRR
BBFFBBFRLR
FBBFFFFRRL
BBFBBFFLRL
FFBFFFFLLL
FFFBBFFLLL
BBFFFBFLLL
BFFFBFBRLL
FFFBBFFRLR"#
        .split('\n')
        .collect::<Vec<&str>>();

    let mut highest = std::u32::MIN;
    for i in &input {
        let id = find_airplane_seat(&i).2;
        if id > highest {
            highest = id;
        }
    }

    println!("Day 5 - part 1 result {}", highest);

    let mut ids = input
        .iter()
        .map(|&s| find_airplane_seat(s).2)
        .collect::<Vec<u32>>();

    ids.sort_unstable();

    let mut values = Vec::new();
    for i in 1..highest {
        values.push(i);
    }

    let seat_id = values
        .iter()
        .filter(|&&val| !ids.iter().any(|&v| v == val))
        .filter(|&&val| ids.iter().any(|&v| v == val - 1) && ids.iter().any(|&v| v == val + 1))
        .collect::<Vec<&u32>>()[0];

    println!("Day 5 - part 2 result {}", seat_id);

    let input = r#"necytxmlfhsu
    uecosjvlhpmk
    
    mnslbuziphawkxyg
    whpmnesikaglzb
    
    kaw
    akw
    
    qurg
    hrqug
    qrgu
    gruq
    
    sczomkv
    zejkhvslmucbwdo
    pxsianovzcmk
    tcokmvsyz
    ozskvimcr
    
    tfzajkxngwsqrbleic
    tijoqnerxsplwzgabkfc
    ogkezbiwqtaxrscljfn
    xsqauezwnjckbtgiryfdml
    
    piz
    zip
    
    tlp
    npl
    lpt
    
    idbnxuclqgw
    sxfdnlatmbhcowigj
    plzgbwdqkcinx
    ydxinwgcblr
    
    nbuxjwhsrlekfyzpq
    kecyiurjfvqpbxhzsn
    nkxfhpbsuyrqjezom
    tinlhkbjsuydrqpzexf
    pnzraelgquxhydsbjkf
    
    obvmw
    eoczsqun
    
    nhdxqwk
    dnzmlxwhkapq
    whxqdnk
    
    jnqdabgc
    nacqdbgj
    
    buyskiwz
    uazbkwys
    
    mbfxswpli
    uswalhxz
    xnq
    tdcrjgoyekv
    
    fancdpkmtuqy
    zdmenqjpaxftuy
    
    jkul
    d
    o
    
    vhpoxgqsrtce
    hgopxeqcdrstv
    xcvrqoetpsgh
    
    gekpzayfux
    pyegazxukf
    pkxaezhgyfu
    fegypukaxz
    zxpyefugka
    
    yrpfd
    ydsh
    
    smjqcafnug
    mnicqsu
    
    tkxi
    ixakt
    tfncxkmi
    
    kedibluzopjhagvtxf
    afiklbvtjeoudhgpzx
    lavbxpjzgedktoifuh
    tbgpaxjqeoifvklhzud
    zghidtfplxbvkoueja
    
    gtvnqyrmf
    hogpb
    
    cnelsqwj
    atyimdbvfxuozkg
    
    lgybpaeztvhrxducjiqosmnwk
    ivpaygxweszductombhljrqk
    
    grli
    lrig
    sjglri
    girl
    rigl
    
    qnwmxjbpecoyztufidhsg
    ynqdiwhmusgtebjfzxcpo
    
    luasrtmfkgywv
    vgskmufywrat
    
    ceyux
    ycux
    
    nurijfsovkbxdp
    ayqclmthgzwe
    
    xeqjdhsubiovwrg
    gobdjqwvusehrx
    weqjgvhosudrbx
    ngdubsvqxejrhow
    
    ywdfqpxekujbovr
    yujfdoqbkxrepvw
    owfkudjbvyrxqep
    oqjvedypxukwbrf
    xufwdpebyjrqokv
    
    csujqndzmw
    wznscqmjud
    cqpwjzmunda
    wjumnzcqd
    
    dhsow
    ohswd
    wsnogh
    
    cxfltiq
    ixztfq
    gtfqiux
    jtqfxi
    xiftzq
    
    tr
    rt
    rt
    
    pnuztcbegwiksd
    cuewibkptdnm
    nvqdpcjxekiwtauhofb
    pbytreinuwdkc
    
    y
    y
    hejq
    
    rvmwqn
    wvrmq
    
    knxmwecsjlrft
    hnmwzaqt
    dmywnitg
    tmwvquonh
    bwqvtmnh
    
    e
    e
    e
    
    qjtywgdlek
    nhifs
    zvxinh
    
    nzhroemi
    dzruoegimh
    emzoirh
    
    hpfkraltbxg
    gazxlmphrt
    
    jyhugbxqz
    zbnox
    epdilfaw
    
    vmr
    vsfm
    fmvsr
    mnvckiaqy
    
    zegpfuwtkrnoqdh
    eorznfkgpduth
    tkfpzhnredguo
    
    zmqkwxipctney
    ncwzeqpmkiytx
    yptienwzxkcqm
    yeqwcpxtkzmin
    
    csbdyzl
    ejoiwtzqabm
    lzbkup
    zbvprxf
    
    wvpshngbmiftuzrxoyde
    szobihvwpdtylqerc
    phbdzoikvtajyesrw
    
    ymuflnrxspqvkihbe
    eymxrhlfnqksuivbp
    
    rktwzd
    wtdkzir
    wkmaziqdrt
    togpzwdkr
    
    uwiazbylcpojst
    lcyibajuptwzso
    
    dtoqbcxhlyrpfwjnkasugvm
    wunkfstmcjhqxliogrdvpy
    
    wpnk
    hdpkwuftjymv
    cpkownz
    kpwz
    rqpwnk
    
    tfplznqcju
    puqlsnzfjtv
    jqtfpzlun
    ntqpczjflu
    
    ozalwumpqv
    ztheoalrwsciyk
    
    wyiqbsr
    bysqwri
    yrbswiq
    iqwrsyb
    iwbrysq
    
    elnroc
    zmjlre
    vhelbru
    jyhruezflvk
    
    ftyceboh
    ofybh
    
    jvlbdm
    noakrg
    
    dokjncqauebfpmszvtl
    suebonpdjmflaqctkvz
    dtjzmcblsvokpunaefq
    tjzmeqkafluvpbconsd
    
    qhrmvegsicnyupf
    uifknverhsg
    izlrvgefndjwasuh
    
    syxvtjwfucilqogmnpk
    jwkytpqofgimvcsnlxu
    dwhglcxzoqnmjfkuiyvpst
    ygwntxipqlmvkufcsoj
    
    hdnqaburow
    hqwadmbocsgue
    bwoqtuadh
    adqhbwuo
    
    dzfohqe
    ehfqzod
    fdzhoqe
    hdofzeq
    
    sxmkrghyvlwzbc
    yhkmwzxbgsvlc
    
    qdumkytwlfsrja
    drfqukzymilsjaw
    jkwqumylsardf
    
    pcvs
    prvc
    hmbpvc
    svrpzc
    
    epkjyc
    jerpoy
    gjpye
    abxepuyj
    ejyopc
    
    ohe
    gheo
    eoh
    ofehy
    
    klhziowmjpyfubdsaqtgvrcn
    aytbormdiksnqpzhcfvulgjw
    lhmvkafdrscjgwnyiuptqobz
    
    vcrqpdxljnmgusie
    cgdwmipluoe
    
    hqibentfmxyszljow
    msozdjnibrfytxqhle
    imlyxwehznjsbft
    ysifkzbljphuxcmenvt
    
    vjhdmkrpecniafgqlstxyzb
    zxjrkebfvcwyqihlsmpgtd
    xrfzidgthsbklmveypqcj
    vjxytzgdlbefsirkcmpqh
    
    dyxrwbngpijkt
    xtwkdipaygjr
    iklrmzwpjxgdtuye
    pdgrxyjkitw
    xtirhvwgdkjyp
    
    zbjdt
    jdat
    shouqjtxcd
    
    klarpuiv
    iqaxjdps
    cpnafbsi
    
    xfpiourcst
    ovfrlutxpb
    trgqfxjuop
    rsktucfpox
    rtufoqpx
    
    g
    g
    g
    
    nfutbyvj
    umvwyi
    
    tjniwlydzqpsxbkcmugf
    utpmixqdvzbfcwgljs
    
    wetypdlnxm
    cfavwyzji
    mywgu
    lyorqkbw
    whdeyxp
    
    qsbnwozepvhkmaxdrt
    qmsopwtaznxfkehdyrv
    xnvhwrzpemkqdtsoa
    ituvkrhmdegazwsobnxpq
    
    fxhwdio
    hofdxi
    hivztfonxdg
    fhkoidx
    
    ja
    jca
    ajzxt
    ahjy
    tajc
    
    epan
    pcn
    udn
    
    mfowepjdlnkags
    oakwgflnp
    pnlzgoawfk
    kfgrowlnap
    
    uw
    aimwo
    wzrhcjeq
    wtf
    w
    
    x
    tvx
    x
    
    tigsjdufrwomv
    vjrstigqudohwkm
    visrjodutwbmg
    
    ety
    exatp
    hicfzqnb
    uvejo
    
    oqlgmk
    kgvqxd
    jzhgkpdqy
    ikegfwatqub
    nlogqkz
    
    sl
    q
    
    rkpew
    eqrwk
    rakwe
    
    r
    r
    r
    rb
    
    u
    pua
    
    pjmhtvd
    drjnpuvib
    djsplvy
    dpvj
    
    caugyqs
    hxwyqst
    sougylqre
    
    krfshpzjmgav
    uwrqidcotlbxhj
    
    mzivctbfjgwq
    lsomkypgajiq
    
    ikoamrpc
    kmociarp
    brkmpiwycoa
    itmckrodpa
    kmipacro
    
    htbpcdqx
    ctpbxdq
    
    gpduzcoiyxhleqrknbaw
    nirehbulapcoxkzwdy
    icuzyxopwhkalendrb
    aokdrxhlcebpyiuwnz
    
    yzlvptdaubjerkh
    iyaoltghebpndkjszr
    kqxjtldzpbcyawh
    dbjtlavzmfhkyp
    
    vg
    sv
    
    cdiaynroehqw
    cqtaridyohewn
    dciornehwayq
    rwhnoaeqicdy
    wqroadniechy
    
    ivstkj
    vgciqk
    qkivg
    vkgi
    
    fp
    pf
    pf
    fp
    fp
    
    xidyqnk
    jbicfant
    zabfhin
    
    oqtys
    stqyo
    syqot
    yostq
    boqytws
    
    gmnqxaictud
    tucidaxqnmg
    pgtquxidcnma
    iagdtcfmquxn
    
    yfhxgw
    rgn
    nsog
    
    wndrqktbmfa
    fwqadkrtbnm
    mabfntkqword
    qbwntakrmdf
    
    elanoudxhbzfrtvci
    ztalefiodxrubvckhn
    ulvabcnthpdzxoifer
    
    omdrvepliwafhjynbscu
    mblrxjsfahocwnvyu
    umcjlsowatvnfybhr
    uahvcnbwlmsjzfoyr
    
    n
    tn
    n
    
    grk
    rgk
    krg
    gkr
    rgkhv
    
    stmlnek
    pgsltne
    
    alcztk
    bctvk
    khcgtb
    
    nsgevcthrpykbwfoizda
    khneritgszvbfycaowdp
    ivybwzeaogdsrnfctphk
    
    e
    e
    e
    
    cxkzdbgiqyawnm
    afyvkbeirtjowmcl
    
    omb
    p
    
    lnhwcoiyafs
    soihnyxqfc
    ohncjyifsq
    
    o
    o
    om
    oxqnsd
    oaj
    
    okucjhiq
    xpgwcnqjiomyfluk
    irhoukdsbjqc
    
    uolgz
    nglczu
    zlngu
    zmhlgvjru
    
    mhdapszf
    anmfwdvkc
    dpamf
    
    aqleivtyzsogjc
    syvbgxqpdflezoac
    ilvzqsjaegokyc
    
    zxndhja
    haxzd
    xhazd
    
    mo
    om
    mo
    mo
    
    vfxep
    xfepv
    xevpf
    vexfp
    pevfx
    
    mrtkehqnjxizgp
    kbunrqytcmes
    
    nz
    ipneu
    
    bznteo
    tbzeoqwp
    olftzbpk
    grzvityxudohjma
    
    xg
    rg
    
    rdu
    urvd
    rud
    rzud
    
    somnbutz
    tuosnmb
    ntomsbu
    
    fxsvrzqntbueoyp
    eqzsvxyunbroptf
    zvypnrxfsqebuot
    zrosbxeqpunvfty
    zprtfvsnbquexoy
    
    qwzknfvl
    funqlwkj
    nqkwflz
    fgxlkwzqn
    
    rzqihd
    rbz
    zr
    uzer
    rbz
    
    utrq
    osa
    pm
    
    zgdci
    brhu
    
    bxvymlzqkpha
    vmhlakpyxq
    ypxvqkmhal
    xhvpymaqkl
    aplqvkhmyx
    
    sjntbvyogeplwzu
    rbjqiefhapxodwk
    
    frlsoptqwbc
    ljrzxqntfom
    
    zlhvjamtpfonk
    lcokjzdhmfvnp
    siohzynkmbjvlrf
    
    osvgwnticelpzm
    jvswetpozlcg
    
    npmgrvysfj
    nvqxrjfsgmp
    vofthkcjsblpmudizwr
    
    oiev
    lmeiov
    oiezyv
    uievodkac
    
    iforupqjzykdtc
    yqfhdcxvtopz
    tzqpfdycgo
    ceyqfozvtpd
    dpxqflohzyatc
    
    srz
    zsrt
    strz
    xzs
    sz
    
    dgo
    odg
    ogd
    dog
    qogd
    
    gotwqhick
    okhqtgicw
    gsouwitkjcqh
    hctgioqwk
    tkghocwiq
    
    mfnziwvxtlpubqryojasd
    mzojuctvnbriypwflskqda
    pmuavqjifnlhrtbyzwods
    
    ozj
    joz
    zjo
    jzo
    
    jzwbhrscm
    rzmhjctb
    zbrhcmj
    
    sucgimtr
    myliojcrsg
    ipcsegxrmv
    
    znhvi
    hvin
    wosganie
    
    wptoqxmafhg
    awqfhgcozmtx
    
    azbqlviyrwjc
    ybxcrawv
    
    qa
    vcuwbfgsl
    hkraj
    
    biglkjtpoqs
    zvxcfumeywrhand
    
    ske
    s
    sp
    hs
    
    n
    yn
    n
    n
    n
    
    lrvsmxzkgi
    zgrmkdsxl
    lzgmpakrs
    wgqtrkmsfz
    nirmkugzds
    
    crewvjdqsplkibthxz
    wxjmtpirhalqsckbdz
    
    iecxrbplho
    xhrpibovlce
    brxlgchopei
    cbhioprexln
    lchxirebop
    
    f
    f
    q
    
    nxqkrhdi
    pkhdrquxn
    ynqdlxkh
    xghkndq
    
    wvknrdjx
    vknxrwdm
    kfvicpwndb
    
    rcqwlutsanpdz
    yoxenmkvbjf
    
    izqsa
    sqzai
    zqsai
    osiazq
    izqsa
    
    ysokhbpzcqanmdifw
    kdlowyusmqhbjxci
    hibkmzocndqswy
    
    yjwd
    bgas
    yj
    k
    
    tzruq
    uorz
    zckrvux
    
    rjn
    nrj
    nrj
    jrn
    njr
    
    ecu
    pzus
    wokxu
    ruw
    twu
    
    fcnajztduyripl
    chmpkvlynsxrauobji
    
    cwfgtjre
    ejwufr
    frwje
    newvryfjbhq
    iwjextrf
    
    ltqapiwhzfeoncxdg
    hndclgoxeiwtpqfz
    cwnpdogxteilfqhz
    
    uerinxhsygpbzcdvfmo
    lxeihducozgqnrymb
    edmugnyxifoczrbha
    ucezhnrsdybmiotgx
    
    xuvfhjeyi
    ztcgaqs
    
    yxrospmijwvcdb
    yjpbrcosuv
    
    viuhdrgnlak
    lniaruvkdgh
    khrgdnuiavl
    
    qfzxb
    gfqx
    
    k
    nzyo
    kcj
    
    bmi
    bmi
    imb
    imb
    ubmi
    
    pl
    pl
    pl
    lp
    
    x
    v
    x
    
    hgwtocs
    gdctwhoi
    iczwfgtoxh
    
    wrcjdaqm
    wsrlog
    jrnqmcw
    
    ouhmpnyskxcqafidbjvgt
    dnimvbpytfsoagucjx
    jpctsmuxiaovyqnfdbg
    gzbvfwmojuyxsrdtpcna
    yxfolphstnabcmuvjgd
    
    nlrdxe
    ceksxuhazmjo
    ywxedfgqtpr
    
    ckuwoghtaxrledvzms
    mtksozduwvrehxclag
    xwvcusrmlazoetkdgh
    regwvshxkdzuctomal
    
    khsnymbpjz
    hzknspmbj
    
    zqc
    kpgsjrqxiyd
    wtbuhoq
    qwc
    eq
    
    ljewzy
    qlwjy
    wzljy
    lwyj
    ljwy
    
    vpodmhni
    bvryknfg
    jcutez
    
    wedfcoh
    wigfnjkaoz
    exwoqfm
    pfuohsw
    
    adyzjwgqxv
    qjwvxzagyd
    
    cazpofbyd
    nodypfbcam
    
    ukvrcslyqzamhteowdfibx
    ktrgqpmnawdycsivhzlbefx
    fxdabuylcveksjwrmzqhtio
    
    xyb
    xesy
    xdqcy
    yex
    xy
    
    hotd
    odhzqt
    othd
    htod
    
    hvkc
    lzei
    asw
    a
    d
    
    q
    q
    q
    q
    q
    
    a
    a
    a
    
    suqbvedipgzcjykfmaxno
    jzfudeoxikpbvmgynsha
    iswkepmodzvfgxnylujba
    
    xpqjonkzlrfhtyw
    fjqksrlozypwnmxth
    pkrohjnxlfzyqtw
    
    pxuhsc
    hcxsnu
    hxcse
    cxhws
    hcrsex
    
    iglfosahy
    hizgfalodbs
    lgfoiahs
    gsfalhyio
    
    xrgzwbdnohfeijacsmltqp
    djpgqrimtwcafsxnlebzoh
    
    qlrn
    qrn
    qrn
    qrn
    
    xhbiakuzcjm
    nfpykow
    klfs
    
    lcubkpfhnomgwyarjetqd
    wtcrgofdnlpbaykquevhj
    
    bdqusvnyetklmwizcopxjr
    drbxlptyeqcvjwiusmzokn
    oqcjzvuwslkpedbxmrntyi
    jvlnxtdpkusyczbeomqirw
    wltriuqnpzcxbysvojdmek
    
    tyeorbzjdxpqn
    oprlqdnwbzyx
    brnydoqzxp
    
    bupvcrylqazehoknjtf
    aqfroczbylpvuejk
    ubqvelyfrzoxcpagkj
    
    awbzx
    axuw
    ywildxoa
    uwxnba
    
    mjfvro
    wmrfjov
    ojvmursf
    
    zwxouqy
    ouzyw
    
    dsxf
    sfdti
    syrfd
    
    zmibfjk
    kimbz
    
    nebq
    beqn
    eqnb
    bqnpe
    
    avtnckldjpeys
    scjtklpdneyav
    kytncdpsvaelj
    tkpacjysdnvle
    dycsvtjplenak
    
    mkyojv
    biwro
    botsx
    
    owxyu
    wyuxo
    yxwuo
    
    qgjplcvo
    vgjqocpl
    ojvlcpgq
    
    xczvkuohisrfqbagp
    prbeilfzognvqws
    voitfqpgjmrzsb
    rvlfdoqigsbpz
    ipqogbrzfmsv
    
    j
    s
    st
    t
    pzde
    
    kuwbthejxcfrzgosmn
    xwcukhrtsgzf
    tkfcguxwshrz
    hxukfzgdwsrqict
    zwucgkfqhvxtrs
    
    wvthikzpgrjaf
    swdnemkqobcyx
    
    lewoquaprbkxyj
    xpelwoaksubcrhqd
    gbrxmluktiwnaopqe
    epxuyczokafblwjrq
    
    mcyepsvwxr
    bhe
    nqeztgd
    uaehil
    
    tq
    be
    e
    
    duqgair
    dfrwuiqga
    qaugidr
    
    hmofng
    hgnomf
    mofghn
    gmotfhn
    
    ewhblqnsmjict
    whmbxvpjs
    ufzyaogdkr
    
    jqpynazsfvklied
    sjilykvqza
    izsvjlqayk
    qiksvzlajy
    
    vbps
    vbstp
    evnigorks
    vbst
    jslv
    
    qgdzylwkbfou
    jbwygulq
    qwhxrugpeb
    giqdfvbwu
    
    jubto
    aeuvgdpx
    unl
    yorqui
    uqm
    
    ic
    kci
    a
    i
    gyfe
    
    tjxp
    xtj
    xjt
    
    ldspeai
    eapidul
    peiadl
    ipelad
    
    oymwvfriux
    moxuywvfir
    rmvxwoujifyb
    fpvwdumgyroxzi
    
    mcplvnkgjfz
    idszwngkfcvpl
    njzfklgcpvo
    fcvokpnlzg
    
    xdnsilr
    irnlsx
    
    bndvimefz
    atopdu
    xhswykcqgrjl
    
    kvlorsye
    
    vsdoq
    dvos
    vodjs
    
    bxczyfeajkmloi
    jcimfkaloybxze
    ceijaokfzyxmlb
    feymoacbjxikzl
    kcxmiebzyaoljf
    
    lo
    ol
    
    etxkfsgjlqoy
    fgshieyj
    esywfgj
    gwrjsyepf
    
    jtzlgakxins
    gqlznxfabskijp
    ikaxgsnzljh
    lgaizhkjxns
    hgaznkjxtlis
    
    urhjiycw
    rhycisw
    zrwihkcy
    
    xc
    cvx
    cx
    cxgl
    
    audxneqrchg
    efkmodwqplrzsiyb
    
    g
    x
    x
    
    ziuplaqo
    czqtihuw
    zufiq
    nqzubpixak
    
    cibdku
    dbckiu
    kdciub
    
    eshpjzgtwrka
    tgwpafmsjkrehz
    sekbwtnxjhpzrag
    
    dj
    dm
    dj
    
    trqkdpwoxjfyg
    csu
    ancv
    lui
    ebhmaz
    
    ljbvexnhgwfs
    hfjbxneyilwgu
    znejqblcdhfgwx
    
    b
    b
    b
    b
    b
    
    idxrcyjegnu
    lrhvskceg
    rozwmgafbce
    
    gzasrwjlukomvdxhbpc
    cahmgzurlvpwxbdjs
    uxjzsbgmdlchvprwa
    hramvdjxslubpzgcw
    
    gw
    wge
    
    fpgdzy
    wpfdbz
    zpfahde
    lfdpszcorx
    zfdgbqpha
    
    gfuclekqavitdxr
    euwkvdirqtcxa
    riuxvaqkcdte
    
    eqaphotimfdrukcgwbyx
    dxtowkygamibqcrfeuhp
    oiutpbavhdyqergfcmxws
    ahcteowdfrkugpibqjymx
    
    pmixjle
    uxljemp
    lkpsej
    
    mzqcwxuo
    xoucwzmq
    xcumzqsow
    zcuqxmwo
    
    wt
    wt
    
    tyrajn
    hdtgroynjae
    jrnyat
    
    uqifnlv
    iqnevl
    vqienl
    navciql
    
    qjxpshtuzbmfnvogcalw
    skvzqthufxcjeagrmdiwlyb
    
    lvaftw
    vkfaywtl
    lfvawtq
    
    wpusqdfkybg
    mghtdxkpsur
    dsgxhumpk
    jmdkpzrsixug
    
    qcbalvykixze
    fmrhcsadlnio
    cpautjihl
    clgainupm
    ilcaft
    
    pklutwg
    gdpkrl
    lrpkg
    
    wbup
    bygfurw
    ubw
    
    xsoqjceyhunvat
    jshgokqucxvlyn
    
    pyk
    pyk
    
    l
    s
    
    m
    m
    hm
    
    msultwhivz
    vzitlhwmus
    htsmluvzwi
    ihszwmvult
    iwthzsumvl
    
    wlohkegqpb
    jzguytqrncd
    igqvesfa
    
    h
    hgy
    
    xb
    x
    x
    x
    x
    
    ij
    ji
    hqdl
    
    k
    k
    fkswyr
    zk
    
    frvpln
    vfcnlpr
    vfnrlp
    rnpvfl
    
    ouy
    yu
    uy
    yu
    yug
    
    rhekb
    etnxfymlgdwovqzu
    csbheji
    beca
    
    vj
    nfpv
    v
    v
    
    ruvqehmija
    mdhjqrayuie
    ksqrfgjeihpmlu
    
    kbsnlqxfrizjauthodp
    uxdjbqniholafzrskt
    eaokjungrdfshlbtzqix
    wjlsbciontzkdqfxhuar
    qjlosubzkfitnahrdx
    
    sbzcefm
    zsmb
    szmb
    mszb
    smzb
    
    xzisbpmc
    prihnxstmfzb
    klpbmszijxw
    vmepbiszxaq
    zipmjbxvs
    
    qjgbnvamedwi
    qajzdoksweb
    
    gtkfqwlzashcen
    geiqzsnhkfwctv
    esnmhfzgqutpwyk
    igselxqndbfwthkz
    
    obpdyswkvthxgreanmjluq
    lahobnkyxtdrqgseuvcj
    
    pw
    wpe
    ylp
    ep
    
    owrlgniy
    iljkvwxyhopufzsqdc
    
    wxzgdpt
    saxprdtkf
    qdxjotpgbwl
    xtdhjpl
    utdjphxci
    
    gwkrjnizdxbvc
    hfnpygvaw
    ygvutnaw
    
    pecfzghqxturdijsblmk
    klsxuqpfhdbctrjgemzi
    dtuzljfxhmbgciqkeprs
    
    iznfycwdrh
    zwnrchiydf
    
    xutwndim
    osidj
    
    knwyjicb
    jc
    zjgc
    cju
    
    jwtep
    hjer
    egj
    
    khwsitfnxyep
    iwkpnxeh
    wephiklnx
    hcigwzkapxremn
    nwehmapkxib
    
    vuqbmineogja
    dlrpainmvugj
    ravsmgijfcuhynt
    awvpiunjmg
    
    zdcalubhniv
    nclhvibudaz
    vhcdbuznila
    
    bzcaorhqflg
    jteyvpwsuxdnik
    
    icr
    rvic
    cri
    ric
    ric
    
    bqikw
    ibwqk
    
    ipxdcvmfulwejyg
    nqcelgydaskrxhi
    
    pkovrgdljyh
    vq
    ixwv
    vsxaczn
    
    ix
    xi
    ix
    
    hkxl
    hklx
    uxlkcsvgeh
    khlyjx
    blkxh
    
    lhgcae
    yczfas
    conasulj
    xgbflezcuayo
    mvcidtapr
    
    iwaepxckbvjnzmqft
    pwiancxtmufqsvzj
    cpanmiftxjwqzv
    mhftvjnxzpowciqa
    
    ir
    rd
    ir
    
    xt
    xt
    tx
    xtk
    tx
    
    rdatjzibwsu
    mdsuarwtizjb
    
    rselwj
    wetbjy
    pzxqjowe
    njtgbvwmei
    
    qgcazvn
    gqsvcnz
    vczgqn
    
    vmufbi
    efuinvmq
    uicgmkvr
    vnmilu
    ubvwmix
    
    lseymvjuidhkoptazcxrw
    wnbsukhpjoedczrlavgtmf
    scmrptkhdeazojwvquly
    
    qopb
    evwqyl
    
    syuzojncfrliwap
    wnocfplirjzsa
    izwrhvftqbdgpcjoasln
    ljiznraspxofwc
    rzcpanjowslif
    
    lpebrhxtam
    iyuvknwfso
    
    by
    yb
    
    qshtypdlocawvgjb
    xvyrnwfqezshjoka
    
    xhw
    nwhx
    hwx
    whsex
    
    xelcghsquwivjydnrzkbfpa
    elviwcpnsokhjqubgadxyzfr
    dcfqxvneymsutzkrlpgbjiwah
    
    ghckb
    bgh
    
    rdojuvqncia
    nracovyiw
    cvryoina
    echariovn
    
    rjmdqahvbplcusigfx
    cluaqvsfhmrjbpgidx
    gvchlipxjsaufrbqdm
    pjdsqmvlcaubfirhxg
    
    tngbc
    btcng
    nbgct
    bcgnt
    
    aonpsbqkhwclrmdyuz
    mcgubhqdokarzlspnwy
    olhybcuasmrdzqwnkp
    lkshamocyrdznwuqbp
    wlunozascybqprmhkd
    
    skilqth
    qhlts
    uslvqt
    fqstl
    ftshql
    
    fbtxdirelcsqzu
    icsyjfltxdeurq
    rueptfdcqylixs
    usqtxfdrclei
    lwqdvtxeirsfucna
    
    xphvrbl
    bthixl
    wqjmohcfxyazlsb
    vhluxb
    
    kbnogjrhe
    krbngeohj
    hnekjbrog
    rkbeognjh
    kjrohebng
    
    z
    yzn
    
    wslprjfqa
    regwpyqdksaci
    
    glmkujvbatqwc
    mtubqgclavrjwk
    wuqmjlgcabvtk
    galuqbvctmkjw
    ckblwqujgvmat
    
    kgrmfstlejw
    bdithoauynzpv
    
    wvyzhfdistmqkrugx
    nzlspqermfwjahui
    ufqsipwozmhr
    
    fdsavukrbpcm
    zrasokjlvudmpbf
    eqvbkfrswpdtumag
    dmhkfplvbarnus
    
    tdorlvb
    pvdhiko
    omgsjwuavycxe
    pziqbovhnf
    
    f
    a
    u
    u
    u
    
    efg
    egf
    feg
    fge
    
    wsf
    lmcd
    f
    v
    ve
    
    mibscvfqnhoxty
    rxphcsiqbofvnw
    uexoibzljkqagcnf
    
    sbmhgyqvkuoljzwrfnxietca
    ocjebltaqwursipnfkmx
    
    xeh
    unxmh
    
    du
    ud
    ud
    du
    
    tvzo
    vo
    ovft
    vto
    povdq
    
    r
    s
    lhibgo
    qny
    nwk
    
    cwxfhrgjdsyamtkv
    fshogpwdinlrcjexmt
    yxvdtzfrsgmcjqwbh
    
    zil
    liz
    liz
    
    iygvphrujkwaxl
    ikjyutlxawvgrph
    hvuliyrdjakwpxg
    rjthkylwipxgvuad
    wivxkhlausgpjyr
    
    vrxkaqzgmsyon
    vorxcumswqznyag
    
    vlhuwzmdq
    bsoryk
    yeka
    
    eiqwuz
    uvwznpqikr
    wluozi
    xjbyizufsagdtwm
    wzhuic
    
    wydkjlfzpeqtgxorhmnuci
    tuczlfqrdnyxihgjopkwem
    tdpylmxcibgkfhjzwerasnou
    zelpcrdoygnxjvfmktuhwi
    gwmehyiojukcnfxrptzdl
    
    vcuizfrlhqa
    rvizlucafhqj
    cviluqfhraz
    
    wmgu
    ve
    sgly
    fmwx
    
    ke
    ek
    
    elhq
    hqel
    qehl
    ehql
    
    hzdfnwxtjysrio
    mykgu
    vy
    kpmaqy
    gbeykc
    
    pfkj
    hfkpj
    kpfj
    pfjk
    kpfjd
    
    z
    t
    z
    t
    oxy
    
    ux
    auhzoj
    xu
    u
    
    gcaldiph
    lhnfapi
    
    slg
    slg
    lgs
    lgs
    sgl
    
    h
    h
    h
    
    qodwxpmskvjbtycuzghfe
    pesjgxqzbtdcoyhwmu
    ylthszqpuwebodxmgcj
    chqjetzxgdyopbwmuls
    
    fkgtmuyoivdehpjcxsba
    agycjmdsfirvkuowtpbhex
    
    ocaw
    msclaw
    cwm
    jdwceh
    
    ekfyibdncovguazhjr
    jkhgzbaoyidunvfr
    vyaihouznrkgjbfd
    xghojvrbdnzfuyaki
    
    jpgqlhvyi
    pqihag
    hqpig
    gupqhi
    ghqip
    
    glatmnqesbiwzkhup
    jykuqrbswnmvocx
    
    ykqwxpjgcshtdbau
    dfxqwpgbjcstkyua
    xsgatjcbkwdyqup
    usqykjpbgxdcwat
    
    cmevolpwiq
    moagpqevltjiw
    fmwziqbpuvleko
    djvspewmohiql
    
    mdux
    xmkud
    xmdu
    
    vptyrkjxhcmounzgaw
    wkmranuoxcjhslzytdigq
    mjcazrxkguwfhbtnoy
    
    e
    z
    mr
    e
    
    vrqek
    kevrq
    
    wb
    vcti
    hyar
    
    e
    e
    
    chqesv
    vesqch
    eqhvcs
    
    uae
    xzecw
    eyl
    
    bkgftdhvn
    nbfkgdtvhj
    nhftvgpkdb
    
    fwxkbouizcyjg
    kwfqjgcyxzou
    gfkdoytrjpczuwx
    gyjvzcxfuowk
    mxcjzsnuaefowyhlkg
    
    itwzbp
    yhnafe
    
    dtnzjkibprafm
    fnarjbkiptzdm
    njrtpifmkbdaze
    qkndmpaufbrjtiwzx
    
    eknujdxzmytriao
    erkujinftzq
    eskumatjicdnryz
    
    lvzhmxewbgnkuorypfai
    ibnwkrfogvhaezxumpyl
    hvbarmnowygqizujxplkef
    hegopvnwrfybamkluxiz
    
    rdylnbotpv
    raugkqfelbnwy
    
    vk
    gnuypkw
    kvj
    kjh
    
    uprylxio
    luixrp
    ruxplki
    
    u
    gmu
    
    ugbqne
    ykvzwr
    i
    
    vcqosrjbuxtl
    hovtxlrbcqsu
    bequwsltvxco
    vxarnqstcbuzlo
    cvuxqpobtsjl
    
    ctviybroekm
    cyrtbevimok
    tvokrbciyme
    ekctbyvmoir
    tymcviebokr
    
    sdmkxgfhjlpyba
    puaowzsdqthkle
    zwredloshkpca
    
    fjxo
    joxf
    fjxo
    jfxo
    fxoj
    
    mrqtv
    rvqtwfm
    rqmnvt
    wmrvtq
    qrftvm
    
    qwtjlfobuydncgzheisxr
    qwxdlcuhjbnemaorpgfyzist
    
    zenwdhikljm
    nejlmiwzkdh
    jndelhwzkbmi
    zelhwnijkdm
    
    jofdgmy
    ojqdpbfg
    gduojf
    
    awsgplcxqu
    mulxgapdcjqk
    acqjhfldpxgu
    xluqipvbngact
    
    scdzqhyfplnx
    nxkygdjuzbcqsmvt
    zlxsqwnpdcay
    czsxnrqyold
    
    qzvlitw
    njapdb
    xkbcf
    uaogk
    
    ok
    e
    iu
    
    qshidleorzbgjnvpuckyawt
    piojqrtzbnaukgwvchlesd
    gduvqkcnhioepstrljwbza
    qhripvstdjznlbgwuocake
    jevrwsibcqlnutgkohdapz
    
    upcealv
    gmpeyv
    pve
    lpkuevz
    
    pbtldk
    bpqhdk
    cibzd
    
    lk
    hs
    kx
    l
    
    fkuirx
    sy
    zgqw
    qmv
    qv
    
    gdrtbwcjlhmqk
    akdlqzjhctgm
    dchqomkjylaftg
    
    vwal
    ficwjzusyom
    lw
    
    z
    jyxo
    n
    
    x
    x
    x
    twxs
    ox
    
    jzhaxuflebqwsipgctr
    mgawzjihxrpcuesfbqtl
    rljqhausvigxtofwczpbe
    ifwjsdcbztphlxqrkeuag
    hizafygceblrquxwtnsjp
    
    ioamx
    xiaomg
    oxami
    axoim
    amibxo
    
    dxgwsbmlnyqruckezp
    xlurgtqncpwyfbisdkejm
    
    vflkdqcxbzuwmengty
    swdbuexzlqvgmf
    
    ehsmravbk
    ehmvrkadb
    
    qh
    hq
    thqx
    qh
    
    uhi
    aui
    ui
    iu
    iu
    
    wlrnmb
    yblnpr
    bztijfenkola
    
    y
    fgzyvds
    yoj
    iyp
    
    ohbiqfxzp
    fpqihobx
    xhoeiqbfp
    
    cgfi
    figc
    fgci
    cflgi
    cihgfnv
    
    vytbdxlzuwarjphnogic
    gexwbnqfizmhtdjyrcolas
    
    hdeowrivjq
    uzihgmjwxrve
    wijeorhlqdsuv
    ryeitfhvjw
    ejhawvnuir
    
    siaxlobjmkvcwhernytfugd
    rmtfsolejkdxihgvbnwcyua
    oxglvkinrcebswamujfhtdy
    gwsatohiknljburmdycevxf
    cwuzdbgnethrvysmxolfkaji
    
    m
    gm
    bhslre
    
    xonqwkzcfbmtidpyvlsheu
    pylzcoxmvqkehdsnftubjwi
    nworbkczxutyedsvqfhimpl
    
    uatwvlgknhrfoqi
    nwhbtokfiq
    seniqpdzfhxmcotyw
    utoiqafwhn
    nqtoifwjh
    
    oxiflw
    hflwxo
    loxwbf
    
    mbt
    tbm
    
    ocjflngrdakuypqiwxstzhmvbe
    ykdaueiczwglbtrhjqnmsfxovp
    
    wtlzyrcomasqnjude
    owhqdnmkzrpjlsxc
    loqdmszxgvrjwcn
    
    on
    x
    x
    
    lbeyqfxkwm
    jmqnfxbgwde
    oezbufqrtm
    
    d
    y
    y
    ta
    
    z
    p
    
    fyx
    xfiy
    fxyw
    xdfcy
    fyx
    
    csjxkozmnl
    slomjxkzc
    
    hqwadymfsbtlxv
    kdvbatlfshwqmxy
    lyahsxmqfwdtbvz
    wxmlyahfdstvqb
    tvqdsxfynlhamwb
    
    ojmvurikheablq
    ukidqhvaljobe
    
    imc
    tdkjmp
    mi
    
    eq
    eq
    
    thgulyifqobdwvecmarsjnp
    ndlygcmjahefquiswbrptvo
    
    sjnptmdrlc
    uvebzmlodrncp
    lmypxkcfrnsd
    cnmhdlqxiarwpg
    
    nzvqlh
    hlzvn
    lzhnv
    
    juq
    ju
    uj
    
    tjqiucmynvozxh
    kmdvwafrxtq
    exgqmtv
    ftsqxmv
    qklbtmxva
    
    r
    r
    pj
    
    pkoy
    ypok
    kypo
    okyp
    
    f
    fl
    f
    f
    f
    
    snvqdyx
    tdsxvnqy
    deqynxsv
    dnsxqvy
    
    lzhopfimuretwvd
    jqgbvakrds
    
    zfbocen
    mzbnacu
    
    ezyxtajrv
    wzjuxreatv
    
    mrsejiyuvfz
    jkuctbxpoiw
    
    nyvefxqicbah
    brgeyltxvik
    kyuvilexbs
    ztvixyepb
    pxiygvbe
    
    nadq
    nyt
    
    prmt
    ysjqkmocznhda
    gmlibeu
    bxmvf
    
    rm
    wm
    m
    m
    
    wyfjzdlvcgreb
    emlxnkpfjav
    elijupnfvh
    
    pybv
    bpvy
    yrbvp
    
    zrjkxshpufec
    vindgwtqbl
    
    ngmtqzfcpudhw
    tphzgcwnqdfmu
    ftzgphnmcudqw
    quzpmctfngdwh
    gqpdhufwcmtzn
    
    szfcnb
    fczns
    fnzcbsg
    sznvafck
    
    arcezgin
    egcirazn
    gezcrian
    icgnraez
    
    hgqucfnt
    gqnukt
    qgncut
    tuqnhg
    unfgqit
    
    w
    setbdgv
    
    lck
    lxk
    kcl
    kl
    
    emaqhxorctykufp
    tigjzqhv
    
    mlvow
    jqwo
    ghksnruxipoac
    
    hxnpeqt
    htzen
    nhte
    pchtne
    
    ehovjgfzaql
    qhazvjlgeof
    kfgljqhavzoe
    jvlzfhgoeqa
    veolzqfjgah
    
    bdlitrzuwh
    epfmuhgvstibr"#
        .lines()
        .map(|s| s.trim())
        .collect::<Vec<&str>>();

    let mut groups = Vec::new();
    let mut group = HashSet::new();

    for line in &input {
        if line.is_empty() {
            groups.push(group);
            group = HashSet::new();
        }
        line.chars().for_each(|c| {
            group.insert(c);
        });
    }
    groups.push(group);

    println!(
        "Day 6 - part 1 result {}",
        groups.iter().map(|g| g.len()).sum::<usize>()
    );

    let mut groups = Vec::new();
    let mut group = HashMap::new();

    let mut group_size = 0;

    for line in input {
        if line.is_empty() {
            groups.push((group, group_size));
            group = HashMap::new();
            group_size = 0;
            continue;
        }
        line.chars().for_each(|c| {
            *group.entry(c).or_insert(0) += 1;
        });
        group_size += 1;
    }
    groups.push((group, group_size));

    println!(
        "Day 6 - part 2 result {}",
        groups
            .iter()
            .map(|(group, size)| group
                .iter()
                .filter(|(_, i)| *i == size)
                .map(|(c, _)| c)
                .collect::<Vec<&char>>())
            .map(|m| m.len())
            .sum::<usize>()
    );

    println!("Day 7 NOT DONE YET");

    let input = r#"acc +18
    nop +222
    acc -16
    acc +28
    jmp +475
    acc -6
    jmp +584
    acc -12
    acc -8
    jmp +554
    acc -9
    acc +12
    acc -16
    acc +27
    jmp +336
    acc -4
    jmp +214
    acc +38
    jmp +61
    acc +3
    acc +28
    acc +5
    acc -19
    jmp +584
    nop +206
    jmp +506
    acc +36
    jmp +133
    acc +20
    acc +43
    acc -18
    jmp +409
    acc +24
    jmp +131
    acc -12
    acc +7
    acc +7
    jmp +454
    acc +37
    acc -6
    nop +558
    acc +31
    jmp +124
    acc -15
    nop +201
    acc -7
    jmp +297
    acc +3
    nop +517
    jmp +221
    jmp +211
    acc +28
    acc +35
    jmp +5
    acc +31
    nop +325
    acc -15
    jmp +116
    jmp +1
    nop +333
    acc -2
    acc -5
    jmp +138
    acc +19
    acc +9
    jmp +180
    acc +18
    jmp +228
    jmp +495
    jmp +382
    acc +20
    nop +414
    nop +139
    acc +33
    jmp +171
    acc -10
    jmp +41
    acc -2
    jmp +80
    acc +20
    nop +451
    acc +2
    acc +24
    jmp +102
    acc +1
    acc -11
    acc +9
    acc +38
    jmp -73
    acc +17
    acc +16
    acc +12
    acc +43
    jmp +168
    jmp +286
    acc +6
    acc +6
    jmp +271
    acc -17
    acc -5
    acc +1
    jmp -50
    acc -9
    acc +6
    acc -2
    acc +33
    jmp +385
    acc +18
    acc +24
    jmp +370
    acc -5
    acc +23
    acc +6
    jmp +98
    acc -10
    acc -16
    jmp +329
    nop +41
    jmp +463
    nop +224
    acc +35
    jmp +345
    acc +34
    acc -18
    acc +5
    jmp +177
    nop -57
    nop -80
    acc +20
    jmp -12
    acc +24
    acc +39
    jmp +363
    jmp +253
    acc -14
    acc +0
    acc +22
    jmp +118
    acc +43
    acc -2
    jmp +300
    acc -14
    acc +8
    acc +47
    jmp +271
    jmp +420
    acc +33
    acc +15
    acc +20
    acc +25
    jmp +84
    acc +41
    jmp +420
    acc +25
    jmp +238
    jmp +1
    acc +14
    jmp +415
    jmp +68
    jmp +262
    acc +34
    jmp +346
    acc +39
    jmp +56
    jmp +364
    jmp -133
    acc +13
    jmp +1
    acc +33
    jmp +408
    acc +29
    acc -4
    jmp +319
    jmp +106
    jmp +228
    acc -8
    acc +8
    acc +22
    jmp -146
    jmp +223
    acc +27
    nop +191
    acc +49
    jmp +331
    jmp +39
    jmp -170
    acc +28
    acc -6
    acc +50
    jmp +268
    acc +41
    nop +254
    acc +28
    jmp +269
    jmp +140
    acc +10
    nop +131
    acc +3
    jmp -40
    jmp +373
    acc +47
    jmp -91
    acc -19
    jmp +300
    acc -2
    jmp +1
    acc +44
    acc -11
    jmp +306
    acc +33
    jmp -15
    acc +9
    jmp +1
    jmp +144
    acc +40
    nop +184
    nop -75
    nop +228
    jmp +296
    acc +22
    nop +364
    jmp -214
    jmp +18
    jmp +375
    acc +22
    jmp -67
    acc +8
    acc -17
    jmp +174
    jmp -99
    nop -45
    acc +7
    jmp -213
    jmp -218
    acc +50
    nop +52
    nop +98
    jmp -142
    acc +18
    jmp +252
    acc +36
    jmp -194
    acc +1
    nop -53
    jmp -127
    jmp +327
    acc +7
    acc -9
    acc +39
    nop -127
    jmp +84
    jmp -117
    nop -29
    acc +43
    jmp -216
    acc +25
    acc +16
    acc +40
    nop +122
    jmp +140
    jmp +180
    acc +42
    acc -5
    acc -14
    jmp -84
    jmp -31
    acc +37
    acc -11
    jmp -217
    jmp +210
    jmp +170
    nop +301
    jmp +309
    acc +6
    jmp +135
    acc +6
    nop -123
    acc +17
    jmp +315
    acc -1
    nop -46
    nop -58
    nop -59
    jmp +202
    acc +48
    acc +38
    jmp +86
    acc -4
    acc +33
    acc +28
    jmp -50
    nop +43
    acc +38
    acc +13
    jmp +33
    acc +4
    acc +6
    jmp -78
    acc +22
    acc +7
    acc -9
    jmp -56
    acc +30
    nop +54
    nop -81
    nop +198
    jmp +252
    jmp +1
    acc +6
    acc -10
    acc +29
    jmp -242
    jmp +1
    acc +42
    acc +34
    acc +22
    jmp +231
    acc +29
    acc -10
    jmp -161
    acc +37
    acc +9
    jmp -77
    acc -15
    acc +32
    acc +32
    jmp -6
    acc +0
    nop -124
    nop +174
    jmp +20
    acc +45
    acc +24
    jmp -13
    acc +6
    acc -10
    acc +23
    acc -15
    jmp +34
    acc +5
    acc +38
    acc +42
    jmp -116
    acc +0
    acc +8
    jmp -243
    acc -18
    acc +25
    acc +1
    jmp +158
    nop +65
    jmp +1
    jmp +151
    acc +12
    acc +12
    jmp +1
    jmp -305
    jmp +29
    jmp -263
    acc +33
    jmp +1
    nop +142
    jmp +78
    acc +41
    nop -141
    acc -9
    acc +5
    jmp -245
    jmp +41
    acc +16
    nop -83
    jmp -28
    nop -149
    acc +38
    jmp -15
    acc +7
    nop -329
    acc +5
    acc +21
    jmp -7
    acc -19
    jmp -38
    acc +5
    acc +3
    acc +10
    jmp -181
    jmp -240
    acc +19
    acc +15
    acc +31
    acc -11
    jmp -340
    acc +12
    acc +46
    jmp +127
    acc +12
    acc +31
    acc +30
    jmp -158
    acc -10
    jmp -374
    jmp +50
    acc +43
    nop +42
    acc +19
    jmp -232
    acc -14
    acc -4
    jmp +95
    acc +23
    acc +49
    acc +31
    nop -139
    jmp -272
    jmp -141
    acc +26
    acc -8
    jmp +173
    nop +145
    nop +133
    jmp +164
    acc +7
    jmp +23
    acc -4
    acc +48
    jmp -138
    acc +4
    jmp -389
    nop +156
    acc +44
    acc +40
    jmp +146
    nop -247
    acc +44
    jmp +1
    acc +28
    jmp +95
    acc +13
    acc +2
    jmp -254
    acc +24
    jmp +122
    acc +39
    acc +0
    jmp -12
    jmp -179
    nop -44
    nop +100
    acc -19
    nop -47
    jmp -107
    acc +32
    acc +33
    acc +42
    acc +6
    jmp -366
    jmp -122
    acc +2
    nop -443
    nop +72
    jmp -381
    jmp -446
    jmp -332
    acc -7
    acc +45
    jmp -355
    acc +27
    acc -4
    acc +3
    jmp +96
    acc +45
    jmp -402
    acc +45
    acc -3
    acc +22
    jmp -141
    acc +29
    acc -1
    jmp +29
    acc -1
    acc -10
    jmp -208
    acc +6
    nop -196
    jmp -218
    acc -12
    acc +49
    nop -137
    jmp -430
    acc +21
    jmp -110
    nop -287
    acc -3
    jmp -42
    jmp -487
    acc -16
    acc -1
    acc +7
    acc +39
    jmp -119
    jmp +1
    acc +9
    jmp -23
    acc +27
    jmp -300
    acc +12
    jmp -440
    acc +2
    acc +38
    acc +12
    jmp -84
    acc +25
    acc -14
    jmp -418
    acc -15
    acc +48
    jmp +1
    nop -383
    jmp -365
    acc +47
    jmp -193
    acc +23
    jmp -235
    jmp +1
    acc -4
    acc +35
    nop -64
    jmp -87
    acc +32
    jmp -339
    jmp -479
    acc -4
    acc +32
    acc -10
    jmp -77
    acc +0
    acc +47
    acc +41
    jmp -308
    acc -8
    acc -9
    jmp -229
    acc -14
    acc +24
    nop -380
    acc +49
    jmp -174
    acc -11
    nop -69
    jmp +3
    acc -14
    jmp -89
    jmp -301
    acc +46
    acc +8
    nop -156
    acc +44
    jmp +1
    jmp +26
    acc +17
    acc +23
    acc +6
    jmp -4
    jmp -97
    jmp -324
    acc +2
    jmp -27
    nop -195
    acc +3
    acc -13
    acc +15
    jmp -19
    acc +30
    nop -318
    jmp +19
    nop -72
    jmp -315
    acc +4
    nop +6
    jmp -384
    jmp -505
    jmp -512
    acc +33
    jmp -168
    jmp -443
    nop -519
    acc +7
    acc +41
    acc +15
    jmp -269
    nop -539
    jmp -416
    jmp -326
    nop -221
    acc +14
    jmp -186
    acc -1
    jmp -295
    acc +29
    acc +43
    nop -436
    nop -421
    jmp -123
    acc +13
    acc -11
    acc +12
    jmp -155
    acc +9
    acc -16
    acc -15
    nop -380
    jmp +1"#;

    let instructions = input.lines().map(|s| s.trim()).collect::<Vec<&str>>();
    let mut cpu = HGCCpu::new();
    let mut stackpointer_history = HashSet::new();
    let mut last_accumulator = 0;
    loop {
        cpu.execute(instructions[cpu.stackpointer as usize]);
        let res = stackpointer_history.insert(cpu.stackpointer);
        if !res {
            break;
        }
        last_accumulator = cpu.accumulator;
    }

    println!("Day 8 - part 1 result {}", last_accumulator);

    let input = r#"18
19
46
14
29
45
40
47
25
43
36
22
21
4
32
33
37
38
26
2
42
15
5
13
31
9
6
30
7
14
10
8
69
11
12
25
16
17
22
19
18
20
51
21
24
23
26
15
60
13
29
27
44
42
28
38
30
36
31
32
33
34
35
37
55
39
40
70
41
43
78
58
73
101
60
56
57
75
59
67
71
61
68
110
69
80
72
112
76
79
144
129
114
84
99
113
132
115
148
116
118
120
128
126
187
130
190
152
229
141
151
155
299
209
163
239
183
197
331
212
228
270
231
234
236
244
246
355
256
392
271
342
292
360
296
306
318
346
602
375
380
395
534
456
440
459
470
502
478
480
490
517
527
548
563
1082
1136
896
1044
614
693
698
988
755
1065
1352
835
899
915
942
929
1173
958
1043
970
1007
1075
1090
1529
1750
1663
1307
1312
1533
1369
1391
1705
1590
1654
1734
1764
1805
2923
2649
1936
1899
2633
1928
2681
1977
3956
2165
2981
2619
3926
2676
2840
2703
4292
4547
4383
4628
3244
3388
3539
6659
6602
5143
3827
5316
8558
3905
5322
4142
4817
4784
9069
5379
11206
5516
7536
10701
6242
7293
6632
15190
11260
7366
7732
7681
7969
8959
9521
8611
8047
10139
8689
16605
8926
10300
10163
10895
15262
12148
13925
12874
15025
13974
19229
14313
15047
15098
15335
15413
16370
16016
16658
27899
16736
18852
35245
17615
19089
19821
29984
23769
23043
30309
31705
26799
26848
35105
28287
30382
42183
30748
30433
31351
31429
32386
32674
33394
44635
34351
50281
36704
60293
69456
42864
49891
46812
49842
53647
55086
85047
119298
60961
75099
62099
73297
94916
61784
62780
63815
75250
96654
67745
104928
71055
111242
133358
89676
92706
151460
134938
101898
158753
108733
122831
122745
123060
293691
126595
165971
142995
125599
130525
146305
194115
402424
179788
138800
226064
160731
182382
235256
191574
194604
210631
224643
269464
231478
231564
245576
253585
248659
252194
276830
256124
411266
371362
299531
354846
654618
340519
330374
459212
343113
695365
431041
386178
402205
405235
435274
456121
633683
504783
477140
586095
701736
500853
508318
654377
555655
640050
741024
642644
673487
1288060
802325
716552
729291
1497690
1073685
1273733
821452
807440
891395
912414
933261
977993
1117190
1140903
1210054
1009171
1143497
1063973
1445843
1195705
1282694
2428963
1316131
1390039
1518877
2095183
2337706
1536731
1628892
1698835
2076758
1712847
1803809
1824656
1845675
2962865
1987164
2073144
2150074
2152668
2204876
2207470
2981529
2478399
3128369
2908916
4192040
2706170
2926770
3055608
3790973
3165623
3235566
3327727
3502644
5835686
3516656
3628465
3670331
7131109
5941736
5116386
4278020
4913640
4412346
5186405
4685869
7293617
5834539
5615086
6383335
5632940
9001309
5982378
6221231
6401189
6493350
6563293
6830371
8188513
7145121
10907100
10354082
7948351
11998421
8690366
9528732
8963889
15987022
10318809
9872274
13376235
16136864
11248026
12016275
14983687
17469257
12203609
12383567
26433146
12894539
13056643
13393664
13975492
15093472
26270774
16638717
16912240
18492621
17654255
22066601
25440210
18836163
20191083
21120300
23264301
26870031
33085622
23451635
24399842
24587176
25098148
51578243
35095792
34176943
37980840
35460265
27369156
29068964
37160073
33550957
36146876
62568016
47164749
41311383
44778259
45631293
39027246
52333265
55938995
48549783
50321666
77431413
47851477
48987018
65898559
52467304
83311625
68646749
74487511
69011222
80925135
76187319
62619921
115214565
167547830
136864130
91943008
131266670
80338629
83805505
86878723
87577029
100184742
117561005
139346027
116498226
96838495
129912153
101454322
115087225
213051449
150674830
246410379
154562929
216668887
138807240
195425854
142958550
164144134
267732572
167217352
167915658
170684228
185259827
171382534
244412872
184415524
265598456
198292817
211925720
213336721
334233094
216541547
240261562
253894465
369675351
281765790
335133010
323222764
505860018
328218377
354884270
307102684
331361486
337901580
379143072
338599886
342066762
456803109
355798058
382708341
594634061
410218537
536559485
425262441
429878268
547903033
523644231
535660255
585255951
753480818
588868474
630325448
662900742
788164595
635321061
638464170
645004264
669263066
734941130
680666648
694397944
1055587889
1363661010
738506399
792926878
945878792
1277125049
1108900182
1357298686
1298221803
776203571
1275329712
1124528729
1174124425
1343567390
1219193922
1310992096
1759849790
1527868008
1428247939
1283468434
1314267330
1349929714
1375064592
1917455607
2517415725
1531433277
1514709970
2328094104
2959681216
1722082363
2811336442
1885103753
1900732300
2126133285
1950327996
2884378519
2552776668
2549189017
2502662356
2530186018
2594460530
2633398148
2597735764
2658533026
5542911545
2664197044
2724994306
4032125695
3046143247
4048849002
3236792333
3399813723
3607186116
6636606056
3622814663
6048128775
6109848472
3851060296
4076461281
7094992249
5032848374
5051851373
5079375035
5097122886
5124646548
5192196294
5231133912
6843978449
5322730070
5771137553
7268918028
7448662725
6445956970
7122604528
6859606996
7006999839
7022628386
10068771633
14029628225
7473874959
10514926364
13866606835
7927521577
9109309655
10084699747
12705008871
12219727414
10176497921
10316842842
10355780460
12253762298
10553863982
19522680326
21340481794
12630744549
13305563966
13452956809
13468585356
13882235382
14480874798
14934521416
17199126307
19663173637
15401396536
16583184614
17036831232
23021851713
18104019498
32584894296
24022449338
37475406147
30076544308
20672623302
37709454534
20909644442
22807626280
23184608531
25936308515
26083701358
27187799348
28240085382
26921542165
28403106772
28363110180
29415396214
46507126270
48026283817
35140850730
40058682945
41059280570
43480249582
43694475015
38776642800
49268309889
46993345800
41582267744
43717270722
55590906120
49272754622
44094252973
71720334964
49120917046
92815392061
53005243523
54109341513
91506533399
55284652345
79835923370
104405569391
76408742014
91720758832
73917493530
78835325745
85769988600
80358910544
140841675878
124053385559
113302602708
120126012736
85299538466
123190248152
142088146683
125529659060
93367007595
236492850860
107114585036
102126160569
108289895868
130518083527
174235354249
165424026929
140584190811
170556084577
150326235544
161708280480
192137928453
180961486314
205425551202
166128899144
165658449010
178666546061
200481592631
233951198406
307551711771
187425699035
306713089955
201656903463
195493168164
209240745605
366092245096
238807979395
210416056437
366610491775
302292471291
290910426355
374535389217
311140275388
312034516024
331787348154
327366729490
402138496094
344324995071
344795445205
353084148045
387907291666
374159714225
491392018986
434301147559
382918867199
389082602498
503949374754
397150071627
715274021156
419656802042
449224035832
512708527728
612554552531
593202897646
776673885311
685675664605
623174791412
638507004878
736003015244
659154077644
886868241953
733878047703
763981797113
1269787109152
750234219672
763242316723
1248711542972
780068938826
1048236680142
1862990006798
1418575943704
1620746289656
2003665156855
2198644882530
868880837874
2005790124396
1205757450177
1216377689058
1297661082522
1324182669483
3868780131194
1261681796290
1372385052581
1393032125347
1409388297316
1497120364426
1484112267375
1513476536395
1530303158498
1543311255549
1632123154597
1648949776700
1828305618968
2532348947517
2241265890455
2946610859222
2074638288051
2193063507357
2517830614574
2085258526932
2422135139235
2585864465773
2514038771580
2654713921637
2893804950887
2634066848871
2745794063665
2765417177928
4424108109385
2893500564691
2981232631801
7009972575158
3043779694893
3073614414047
3175434410146
6249048824193
4606987235568
4250440758203
4827130356228
4159896814983
7588219867369
4599297298512
7672911712559
4507393666167
10418705776224
5076849060872
5099903237353
5689473181726
5527871799758
5399484026799
14682884287717
5511211241593
5658917742619
7497722523432
5874733196492
6025012326694
6117394108940
7425875168349
7233511229030
7335331225129
8410337573186
9327289819075
10127169098270
8987027171211
12734815251928
19190277953884
9906877692966
22180606811149
9584242727039
12825359195148
10176752298225
15596350874692
10910695268392
11170128984212
11058401769418
11385944438085
11533650939111
11683930069313
14435349899880
13210064421621
28298000920089
13350905337970
21660820037381
16560801048105
16919573952168
20713234257160
18314316990286
18571269898250
24120759690013
19491120420005
19760995025264
20083629991191
20494937995431
20642644496457
29372718759704
21235154067643
31146939463349
31469574429276
39831651859200
31553339764849
23069874507398
26560969759591
45904924329156
50681729449604
32842025757975
30270479290138
31665222328256
33480375000273
34875118038391
55785982018269
40255933020695
36885586888536
38062390318255
39252115445269
39844625016455
40403639521721
47055907755022
67698552251479
41877798564100
53340353797536
59297544385898
49630844266989
54539448936674
62321989952667
54623214272247
83941494643558
72686650774430
63112505048113
113794234497717
123786119660013
68355493038664
88215471835927
98549659831167
71760704926927"#;

    let numbers = input
        .lines()
        .map(|s| s.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let invalid = find_first_invalid_xmas_number(&numbers, 25).unwrap();

    println!("Day 9 - part 1 result {}", invalid);

    let numbers = input
        .lines()
        .map(|s| s.trim().parse::<u64>().unwrap() )
        .collect::<Vec<u64>>();

    let number = find_first_invalid_xmas_number(&numbers, 25).unwrap();

    let set = find_contiguous_set_sum_to(number, &numbers);
    let smallest = set.iter().min().unwrap();
    let highest = set.iter().max().unwrap();

    println!("Day 9 - part 2 result {}", smallest + highest);
}
