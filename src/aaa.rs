use std::io::Read;

use crate::cheats::*;

#[derive(Debug, PartialEq)]
pub struct Mem {
    num: [i32; 1024],
    ptr: usize,
}

impl Default for Mem {
    fn default() -> Self {
        Self {
            num: [0; 1024],
            ptr: 0,
        }
    }
}

pub trait Aaargh {
    fn get(mem: Mem) -> Mem;
}

pub trait Counter {
    fn count() -> i32;
}

impl Counter for () {
    fn count() -> i32 {
        return 0;
    }
}

impl<T> Counter for (T, ())
where
    T: Counter,
{
    fn count() -> i32 {
        return T::count() + 1;
    }
}

impl<T, TT, TTT> Counter for (T, TT, TTT)
where
    T: Counter,
    TT: Counter,
    TTT: Counter,
{
    fn count() -> i32 {
        return T::count() + TT::count() * 4 + TTT::count() * 16;
    }
}

impl Aaargh for () {
    fn get(mem: Mem) -> Mem {
        mem
    }
}

impl<T> Aaargh for (T, Add)
where
    T: Aaargh,
{
    fn get(mem: Mem) -> Mem {
        let mut it = T::get(mem);
        it.num[it.ptr] += 1;
        it
    }
}

impl<T, C> Aaargh for (T, Add, C)
where
    T: Aaargh,
    C: Counter,
{
    fn get(mem: Mem) -> Mem {
        let mut it = T::get(mem);
        it.num[it.ptr] += C::count();
        it
    }
}

impl<T, C> Aaargh for (T, Sub, C)
where
    T: Aaargh,
    C: Counter,
{
    fn get(mem: Mem) -> Mem {
        let mut it = T::get(mem);
        it.num[it.ptr] -= C::count();
        it
    }
}

impl<T> Aaargh for (T, PLeft)
where
    T: Aaargh,
{
    fn get(mem: Mem) -> Mem {
        let mut it = T::get(mem);
        it.ptr -= 1;
        it
    }
}

impl<T> Aaargh for (T, PRight)
where
    T: Aaargh,
{
    fn get(mem: Mem) -> Mem {
        let mut it = T::get(mem);
        it.ptr += 1;
        it
    }
}

impl<T> Aaargh for (T, Print)
where
    T: Aaargh,
{
    fn get(mem: Mem) -> Mem {
        let it = T::get(mem);
        print!("{}", char::from_u32(it.num[it.ptr] as u32).unwrap());
        it
    }
}

impl<T> Aaargh for (T, ReadCmd)
where
    T: Aaargh,
{
    fn get(mut mem: Mem) -> Mem {
        mem = T::get(mem);
        let it = std::io::stdin()
            .bytes()
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as i32)
            .unwrap();
        mem.num[mem.ptr] = it;
        mem
    }
}

impl<T, L> Aaargh for (T, Loop, L)
where
    T: Aaargh,
    L: Aaargh,
{
    fn get(mut mem: Mem) -> Mem {
        mem = T::get(mem);
        loop {
            mem = L::get(mem);
            if mem.num[mem.ptr] == 0 {
                break;
            }
        }
        mem
    }
}

pub trait Arghh: Aaargh {
    #[allow(non_snake_case)]
    fn O() {
        Self::get(Mem::default());
    }
}

impl<T> Arghh for T where T: Aaargh {}

#[cfg(test)]
mod test {
    use super::Counter;
    #[test]
    fn counter() {
        assert_eq!(<()>::count(), 0);
        assert_eq!(<((), ())>::count(), 1);
        assert_eq!(<(((), ()), ())>::count(), 2);
        assert_eq!(<((((), ()), ()), ())>::count(), 3);
        assert_eq!(<((), ((), ()), ())>::count(), 4);
        assert_eq!(<(((), ()), (((), ()), ()), ())>::count(), 9);
        assert_eq!(<((), (), ())>::count(), 0);
    }
}
