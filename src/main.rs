use itertools::iproduct; // Cartesian multiplication of iterators
use rand::{rngs::OsRng, seq::SliceRandom}; // Unexplainably bad bruteforcer for mappings
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

fn test_mapping() {
    let fg = |a: isize, b: isize| {
        let mut r = (a * b).rem_euclid(17);
        while r > 17 {
            r -= 17;
        }
        while r > 8 {
            r = 17 - r;
        }
        return r;
    };
    let fg1 = |a: isize, b: isize| (a + b).rem_euclid(8);

    let mut mapping: HashMap<isize, isize> = HashMap::from([
        (1, 0),
        (2, 1),
        (3, 3),
        (4, 2),
        (5, 4),
        (6, 5),
        (7, 6),
        (8, 5),
    ]);
    loop {
        let mut res_l = String::with_capacity(512);
        let mut res_r = String::with_capacity(512);
        let mut success = true;
        for (a, b) in iproduct!(1..=8, 1..=8) {
            let left = mapping.get(&fg(a, b)).unwrap();
            let right = fg1(*mapping.get(&a).unwrap(), *mapping.get(&b).unwrap());
            res_l.push_str(format!("{left}\t").as_str());
            if b == 8 {
                res_l.push('\n');
            }
            res_r.push_str(format!("{right}\t").as_str());
            if b == 8 {
                res_r.push('\n');
            }
            if *left != right {
                success = false;
                res_l.clear();
                res_r.clear();
                let mut new_map: Vec<isize> = (2..=8).collect();
                new_map.shuffle(&mut OsRng);
                new_map.iter().zip(1..=7).for_each(|(l, r)| {
                    mapping.insert(*l, r);
                });
                // inject assumptions ;)
                mapping.insert(1, 0);
                mapping.insert(2, 2);
                break;
            }
        }

        if success {
            println!("Found mapping");
            mapping.iter().for_each(|(a, b)| println!("{a}->{b}"));

            print!("{res_l}");
            println!("---");
            print!("{res_r}");
            break;
        }
    }
}

fn find_cyclic_subgroups<F>(op: F, r: Range<isize>)
where
    F: Fn(i8, i8) -> i8,
{
    r.for_each(|g| {
        let mut gpn = g;
        let mut set: HashSet<isize> = HashSet::new();
        set.insert(g);
        loop {
            gpn = op(g as i8, gpn as i8) as isize;
            set.insert(gpn);

            if gpn == 1 {
                break;
            }
        }
        print!("{g} produces subgroup {{");
        set.iter().for_each(|x| print!("{x},"));
        print!("}}\n");
    });
}

fn main() {
    const MOD: usize = 17; // UNUSED
    const RANGE_MIN: i8 = 1;
    const RANGE_MAX: i8 = 16;

    let op = |x: isize, y: isize| -> isize { (x * y).rem_euclid(MOD as isize) };

    const T_WIDTH: usize = (RANGE_MAX - RANGE_MIN).abs() as usize + 2;
    let mut table: Vec<Vec<i8>> = vec![vec![0; T_WIDTH]; T_WIDTH];

    iproduct!(RANGE_MIN..=RANGE_MAX, RANGE_MIN..=RANGE_MAX).for_each(|(x, y)| {
        let res = op(x as isize, y as isize);
        let row_index = (x - 1) as usize;
        let column_index = (y - 1) as usize;

        table[row_index][column_index] = res as i8;
    });

    print!("*\t\t");
    (RANGE_MIN..=RANGE_MAX).for_each(|n| print!("{n}\t"));
    println!("");
    (0..=17).for_each(|_| print!("_\t"));
    println!("");
    iproduct!(RANGE_MIN..=RANGE_MAX, RANGE_MIN..=RANGE_MAX).for_each(|(x, y)| {
        let row_index = (x - 1) as usize;
        let column_index = (y - 1) as usize;

        if y == 1 {
            print!("{x:02} |\t");
        }

        print!("{}", table[row_index][column_index]);
        if y == RANGE_MAX {
            println!("");
        } else {
            print!("\t");
        }
    });

    find_cyclic_subgroups(
        |arg0: i8, arg1: i8| op(arg0 as isize, arg1 as isize) as i8,
        (RANGE_MIN as isize)..(RANGE_MAX + 1) as isize,
    );

    test_mapping();
}
