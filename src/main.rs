use itertools::iproduct; // Cartesian multiplication of iterators
use std::{collections::HashSet, ops::Range};

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
}
