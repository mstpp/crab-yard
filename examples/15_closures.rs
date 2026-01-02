// map() requires FnMut() trait, so we need to pass a &T, not just T
// capturing closure > captures base by owning it
fn clsr<T: Clone + std::ops::Add<Output = T>>(base: T) -> impl Fn(&T) -> T {
    move |i| base.clone() + i.clone()
    // for Copy trait, this would be
    // move |i| base + *i
}

// for comparing adding partial ord
fn clsr_2<T: Clone + std::ops::Add<Output = T> + PartialOrd>(base: T) -> impl Fn(&T) -> T {
    move |i| {
        if *i <= base {
            i.clone()
        } else {
            i.clone() + base.clone()
        }
    }
}

// to get rid of move, we can remove owned base
// non-capturing closure
fn clsr_3<T: Clone + std::ops::Mul<Output = T>>() -> impl Fn(&T) -> T {
    |i| i.clone() * i.clone()
}

// or we can specify lifetime
// capturing closure - captures base by ref, (shared borrow)
fn clsr_4<'a, T: Clone + std::ops::Add<Output = T>>(base: &'a T) -> impl Fn(&T) -> T + 'a {
    |i| base.clone() + i.clone()
}

fn main() {
    let v = vec![1, 3, 5, 7];
    let base = 10;
    // v1
    let res: Vec<_> = v.iter().map(clsr(base)).collect();
    assert_eq!(res, vec![11, 13, 15, 17]);

    let base = 4;
    // v2 - adding partial ord trait
    let r2: Vec<_> = v.iter().map(clsr_2(base)).collect();
    assert_eq!(r2, vec![1, 3, 9, 11]);

    // v3 - no input arg for closure, we dont need to "move"
    let r3: Vec<_> = v.iter().map(clsr_3()).collect();
    assert_eq!(r3, vec![1, 9, 25, 49]);

    let base4 = 100;
    // v4 - lifetime for base, no move for closure
    let r4: Vec<_> = v.iter().map(clsr_4(&base4)).collect();
    assert_eq!(r4, vec![101, 103, 105, 107]);
}
