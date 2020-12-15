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

/*
        ("byr", |val| { return (val => 1920 && val <= 2002); }),

        /* "cid" */
*/

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

fn is_valid_passport_with_data_validation(passport: &str) -> bool {
    let required_fields: Vec<(&str, for<'r> fn(&'r str) -> bool)> = vec![
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

#[cfg(test)]
mod tests {
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
}
