use std::fmt::Display;

pub fn sort<T>(list: &mut [T])
where
    T: PartialOrd + Sized + Display,
{
    let len = list.len();
    for i in (0..len).rev() {
        println!("{}", list[i]);
        put_in_sorted(&mut list[..(len - 1 - i)], i)
    }
}

fn put_in_sorted<T>(list: &mut [T], i: usize)
where
    T: PartialOrd + Sized,
{
    let len = list.len();
    for j in (0..len).rev() {
        if list[j] < list[i] {}
    }
}

fn shift_right<T>(list: &mut [T])
where
    T: Sized + Copy,
{
    let len = list.len();
    for i in (1..len).rev() {
        list[i] = list[i - 1];
    }
}

#[cfg(test)]
mod tests {
    use super::sort;
    #[test]
    fn insertion_sort() {
        let mut list = [4, 3, 1, 2, 5];
        sort(&mut list);
        assert_eq!([1, 2, 3, 4, 5], list);
    }
}
