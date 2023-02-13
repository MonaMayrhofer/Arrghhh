pub type Add = ();
pub type Sub = ((), ());
pub type PLeft = ((), (), ());
pub type PRight = ((), (), (), ());
pub type Print = ((), (), (), (), ());
pub type Loop = ((), (), (), (), (), ());
pub type ReadCmd = ((), (), (), (), (), (), ());

type Num1 = (((), ()), (), ());
type Num3 = ((((), ()), ()), ());
type Num6 = ((((), ()), ()), ((), ()), ());
type Num7 = (((((), ()), ()), ()), ((), ()), ());
type Num8 = ((), (((), ()), ()), ());
type Num10 = ((((), ()), ()), (((), ()), ()), ());
type Num47 = (((((), ()), ()), ()), ((((), ()), ()), ()), (((), ()), ()));
type Num48 = ((), (), ((((), ()), ()), ()));
type Num55 = (((((), ()), ()), ()), ((), ()), ((((), ()), ()), ()));
type Num58 = ((((), ()), ()), (((), ()), ()), ((((), ()), ()), ()));
type Num72 = ((), (((), ()), ()), (((((), ()), ()), ()), ()));

pub type HelloWorld<T> = (
    (
        (
            (
                (
                    (
                        (
                            (
                                (
                                    (
                                        (
                                            (
                                                (
                                                    (
                                                        (
                                                            (
                                                                (
                                                                    (
                                                                        (
                                                                            (
                                                                                (
                                                                                    (
                                                                                        (
                                                                                            (
                                                                                                T,
                                                                                                Add,
                                                                                                Num72,
                                                                                            ),
                                                                                            Print,
                                                                                        ),
                                                                                        Sub,
                                                                                        Num3,
                                                                                    ),
                                                                                    Print,
                                                                                ),
                                                                                Add,
                                                                                Num7,
                                                                            ),
                                                                            Print,
                                                                        ),
                                                                        Print,
                                                                    ),
                                                                    Add,
                                                                    Num3,
                                                                ),
                                                                Print,
                                                            ),
                                                            Sub,
                                                            Num47,
                                                        ),
                                                        Print,
                                                    ),
                                                    Add,
                                                    Num55,
                                                ),
                                                Print,
                                            ),
                                            Sub,
                                            Num8,
                                        ),
                                        Print,
                                    ),
                                    Add,
                                    Num3,
                                ),
                                Print,
                            ),
                            Sub,
                            Num6,
                        ),
                        Print,
                    ),
                    Sub,
                    Num8,
                ),
                Print,
            ),
            Sub,
            Num58,
        ),
        Print,
    ),
    Sub,
    Num10,
);

pub type ReadIntCmd<T> = (((T, ReadCmd), Sub, Num48), Print);

pub type HelloWorldNTimes = (
    ReadIntCmd<((), PRight)>,
    Loop,
    (((HelloWorld<((), PLeft)>, PRight), Sub, Num1), Print),
);

#[cfg(test)]
mod test {
    use super::*;
    use crate::aaa::Counter;
    #[test]
    fn numbers() {
        assert_eq!(Num1::count(), 1);
        assert_eq!(Num3::count(), 3);
        assert_eq!(Num6::count(), 6);
        assert_eq!(Num7::count(), 7);
        assert_eq!(Num8::count(), 8);
        assert_eq!(Num10::count(), 10);
        assert_eq!(Num47::count(), 47);
        assert_eq!(Num48::count(), 48);
        assert_eq!(Num55::count(), 55);
        assert_eq!(Num58::count(), 58);
        assert_eq!(Num72::count(), 72);
    }
}
