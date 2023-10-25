#![feature(return_position_impl_trait_in_trait)]

use std::fmt::Display;

trait Lambda: Copy {
    fn eval<T: Lambda>(self, lambda: T) -> impl Lambda;
}


/// Id, \x . x
#[derive(Copy, Clone)]
struct Id;

impl Lambda for Id {
    fn eval<T: Lambda>(self, lambda: T) -> impl Lambda {
        lambda
    }
}


/// True, \x . \y . x
#[derive(Copy, Clone)]
struct True;

#[derive(Copy, Clone)]
struct True_<T: Lambda>(T);

impl Lambda for True {
    fn eval<T: Lambda>(self, lambda: T) -> impl Lambda {
        True_(lambda)
    }
}

impl<T: Lambda> Lambda for True_<T> {
    fn eval<U: Lambda>(self, lambda: U) -> impl Lambda {
        self.0
    }
}


/// False, \x . \y . y |  \x . id
#[derive(Copy, Clone)]
struct False;

#[derive(Copy, Clone)]
struct False_<T: Lambda>(T);

impl Lambda for False {
    fn eval<T: Lambda>(self, lambda: T) -> impl Lambda {
        False_(lambda)
    }
}

impl<T: Lambda> Lambda for False_<T> {
    fn eval<U: Lambda>(self, lambda: U) -> impl Lambda {
        lambda
    }
}


// Lor, \x . \y . x x y | \x . x x
#[derive(Copy, Clone)]
struct Lor;

#[derive(Copy, Clone)]
struct Lor_<T: Lambda>(T);

impl Lambda for Lor {
    fn eval<T: Lambda>(self, lambda: T) -> impl Lambda {
        Lor_(lambda)
    }
}

impl<T: Lambda> Lambda for Lor_<T> {
    fn eval<U: Lambda>(self, lambda: U) -> impl Lambda {
        let first = self.0;

        first.eval(first).eval(lambda)
    }
}


// Land, \x . \y . x y x
#[derive(Copy, Clone)]
struct Land;

#[derive(Copy, Clone)]
struct Land_<T: Lambda>(T);

impl Lambda for Land {
    fn eval<T: Lambda>(self, lambda: T) -> impl Lambda {
        Land_(lambda)
    }
}

impl<T: Lambda> Lambda for Land_<T> {
    fn eval<U: Lambda>(self, lambda: U) -> impl Lambda {
        let first = self.0;

        first.eval(lambda).eval(first)
    }
}


// Not, \x . x False True
#[derive(Copy, Clone)]
struct Not;

impl Lambda for Not {
    fn eval<U: Lambda>(self, lambda: U) -> impl Lambda {
        lambda.eval(False).eval(True)
    }
}


// Zero, false
#[derive(Copy, Clone)]
struct Zero;
#[derive(Copy, Clone)]
struct Zero_<T: Lambda>(T);

impl Lambda for Zero {
    fn eval<T: Lambda>(self, lambda: T) -> impl Lambda {
        Zero_(lambda)
    }
}

impl<T: Lambda> Lambda for Zero_<T> {
    fn eval<U: Lambda>(self, lambda: U) -> impl Lambda {
        lambda
    }
}


// Succ, \x . \y . \z . y ( x y z )
#[derive(Copy, Clone)]
struct Succ;
#[derive(Copy, Clone)]
struct Succ_<T: Lambda>(T);
#[derive(Copy, Clone)]
struct Succ__<T: Lambda, U: Lambda>(T, U);

impl Lambda for Succ {
    fn eval<T: Lambda>(self, lambda: T) -> impl Lambda {
        Succ_(lambda)
    }
}

impl<T: Lambda> Lambda for Succ_<T> {
    fn eval<U: Lambda>(self, lambda: U) -> impl Lambda {
        Succ__(self.0, lambda)
    }
}

impl<T: Lambda, U: Lambda> Lambda for Succ__<T, U> {
    fn eval<V: Lambda>(self, lambda: V) -> impl Lambda {
        self.1.eval(self.0.eval(self.1).eval(lambda))
    }
}


// Add, \x . \y . \z . \a . x z ( y z a )
#[derive(Copy, Clone)]
struct Add;

#[derive(Copy, Clone)]
struct Add_<T: Lambda>(T);

#[derive(Copy, Clone)]
struct Add__<T: Lambda, U: Lambda>(T, U);


#[derive(Copy, Clone)]
struct Add___<T: Lambda, U: Lambda, V: Lambda>(T, U, V);

impl Lambda for Add {
    fn eval<T: Lambda>(self, lambda: T) -> impl Lambda {
        Add_(lambda)
    }
}

impl<T: Lambda> Lambda for Add_<T> {
    fn eval<U: Lambda>(self, lambda: U) -> impl Lambda {
        Add__(self.0, lambda)
    }
}

impl<T: Lambda, U: Lambda> Lambda for Add__<T, U> {
    fn eval<V: Lambda>(self, lambda: V) -> impl Lambda {
        Add___(self.0, self.1, lambda)
   }
}

impl<T: Lambda, U: Lambda, V: Lambda> Lambda for Add___<T, U, V> {
    fn eval<W: Lambda>(self, lambda: W) -> impl Lambda {
        self.0.eval(self.2).eval(self.1.eval(self.2).eval(lambda))
   }
}


/// Add2, \x . \y . x succ y
#[derive(Copy, Clone)]
struct Add2;

#[derive(Copy, Clone)]
struct Add2_<T: Lambda>(T);

impl Lambda for Add2 {
    fn eval<T: Lambda>(self, lambda: T) -> impl Lambda {
        Add2_(lambda)
    }
}

impl<T: Lambda> Lambda for Add2_<T> {
    fn eval<U: Lambda>(self, lambda: U) -> impl Lambda {
        self.0.eval(Succ).eval(lambda)
    }
}

fn main() {

    println!("True - False");
    println!("{}", type_name(&True.eval(True).eval(False)));
    println!("{}", type_name(&False.eval(True).eval(False)));

    println!();
    println!("Lor");
    println!("{}", type_name(&Lor.eval(True).eval(True)));
    println!("{}", type_name(&Lor.eval(True).eval(False)));
    println!("{}", type_name(&Lor.eval(False).eval(True)));
    println!("{}", type_name(&Lor.eval(False).eval(False)));

    println!();
    println!("Land");
    println!("{}", type_name(&Land.eval(True).eval(True)));
    println!("{}", type_name(&Land.eval(True).eval(False)));
    println!("{}", type_name(&Land.eval(False).eval(True)));
    println!("{}", type_name(&Land.eval(False).eval(False)));

    println!();
    println!("Succ");
    println!("{}", type_name(&Succ.eval(Succ.eval(Zero))));

    println!();
    println!("Add");

    let result = Add.eval(Succ.eval(Succ.eval(Zero))).eval(Succ.eval(Succ.eval(Succ.eval(Zero)))).eval(Succ).eval(Zero);
    println!("{}", type_name(&result));
    println!("{}", size_of(&result));

    let result = Add2.eval(Succ.eval(Succ.eval(Zero))).eval(Succ.eval(Succ.eval(Succ.eval(Zero))));
    println!("{}", type_name(&result));
    println!("{}", size_of(&result));
}


fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn size_of<T>(_: &T) -> usize {
    std::mem::size_of::<T>()
}
