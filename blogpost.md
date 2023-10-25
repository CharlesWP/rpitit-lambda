# Taking RPITIT, Rusts shiny new feature for a test ride, with type-system lambda calculus.

With Rust version 1.75 two important new features will be stabilized. The features go by the names
return-position `impl Trait` in trait (RPITIT) and async fn in trait (AFIT). Let me take a moment
here to congratulate and thank everyone working on this features for their incredible work. This
really is a big step in the Rust ecosystem. While the later one is certainly the more `await`ed one
by most people, I will today focus on RPITIT, which is also the foundation for AFIT.

I will provide some code samples, but for the sake of brevity and not to repeat myself, not all
code is explicitly written out. If you want to have a look at the full version, I'll add it at the
end of the post.

<!-- Explain impl Trait as return type -->
## impl Trait in the return type

So what exactly is RPITIT? Rust had the possibility to specify the trait bound in the return type
instead of the actual type since version 1.26. However, one of the limitations was that this syntax
could not be used in traits. With the release of Rust 1.75 this restriction will be lifted.

But for what do we even need this in the first place? Couldn't this just be accomplished with
generics? To answer that question, let's first take a look at the function declaration of
`serde_json`'s `from_str`: `pub fn from_str<'a, T: Deserialize<'a>>(s: &'a str) -> Result<T>`. As
you can see, this function is generic over `T`. But the actual concrete type of `T` is in the hand
of the end user. But what if we want to return a value of a distinct type that implements a
trait but that should not be up to the user? Then we need to use the `impl` syntax.

If you think this is just syntactic sugar to not spell out the whole name of the type, you are
at least partially right, if there weren't Voldemort types. Voldemort types (types who must not be
named) are types that you can not name in a program. In rust, both closures and functions are such
Voldemort types. Each function or closure has its own unique type determined during compile-time.
This means you can not know the concrete type before the program actually compiles and therefor
can't use it to refer to them. But with the return type impl syntax you can tell the compiler to
figure the type out for you. Let's take a quick look at an example:

``` Rust
fn get_iter(start: i32, end: i32) -> impl Iterator<Item = (usize, i32)> {
    (start..end).into_iter().map(|x| x * 2 + 1).filter(|x| *x % 3 == 0).enumerate()
}
```

Besides the obvious error with Voldemort types, this function could have the return type
`Enumerate<Filter<Map<Range<i32>, {{closure}}>, {{closure}}>>`. But that would not really give the
user of our function any more useful information. And if we ever decided to change the function, we
would have to change the return type with it. With this new feature however we can pass a lot of
work onto the compiler so we are faster while programming.

<!-- Explain RPITIT -->
## return-position `impl Trait` in trait

Now with Rust 1.75 this feature finally is also available for Traits. Now you can specify a trait
with impl return types. Each implementation of the trait can then return a different concrete type
without violating the Trait bounds. The compiler than figures out based on the implementation of
the Trait the concrete type that is returned. Since we need to know the concrete implementation at
compile-time to know the type that is returned, this obviously makes the whole Trait no longer
[object-safe](https://doc.rust-lang.org/reference/items/traits.html#object-safety), meaning we
cannot create a Trait object `Box<dyn Trait>` with it.

<!-- Explain Lambda Calculus -->
## Lambda Calculus

Before we can implement Lambda Calculus on the type system, we first need to understand what it
actually is. Lambda Calculus is a model of computation introduced in the 1930s by mathematician
Alonzo Church. If follows a few simple rules: The only thing that exists in the basic calculus are
function. Each function takes exactly one argument and returns exactly one argument. In the
function body, we can only either apply functions to other function or construct new function. The
simplest function like this is the identity function: `λ x . x` which is a function that
returns it's parameter. This function however isn't very helpful. We can also apply functions. For
example the function `λ x. x x` applies the function x once to itself. Multiple arguments can
be modeled by returning a function that binds the first argument. With that we can construct
new functions. `λ x . λ y . x` returns a function that takes one argument `y` and
completely ignores it, returning `x` instead. In the standard notation, lambda calculus will be
evaluated from left to right, so the function `λ x . x x x` is equivalent to `\ļambda x . (x x) x`.
Calculation is then performed by applying the functions until it is in it's minimal form and can no
longer be reduced.

<!-- Get everything together -->
## Putting everything together

So how can we use this to actually calculate things in Rust? We know that we need to store state to
return constructed functions, so each function will be a struct. Then, we need to define the common
interface for our functions. As we have just established, the function needs to take one argument
and return one argument, both of which are functions. The function is also allowed a reference to
itself, which does not count as a function argument.

``` Rust
trait Lambda: Copy {
    fn eval<T: Lambda>(self, lambda: T) -> impl Lambda;
}
```

And here we can already see RPITIT in action. We know that each function will return a struct that
has the lambda trait and we know that that function will be a concrete type. So we leave the
concrete type details up to the compiler.

<!-- Boolean logic -->
## Boolean logic

Let's warm up to the concept by starting from the very basics of computer science: `true` and `false`.
We can't have truth values, as lambda calculus only allows for functions to exist, so we have to
encode these two as functions, e.g. types that implement Lambda. If we think about how we use `true`
and `false`, it's to compare them and change the control flow accordingly. So our two values will
simply be functions that take two inputs and call the respective function: `True = λ x. λ y . x`
and `False = λ x. λ y . y`. In plain english: If the value is `true`, the first function
is called, if the value is `false`, the second function will be called instead.

We also know that we need to store some state here, so we need a second struct to hold it. We will
name each function that carries state for it's parent function the same as the parent function,
followed by an underscore, to know it represents a partial computation. Implemented this logic
would look like this:

``` Rust
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
```

The same, just in reverse is also true for our `False` implementation. As you can see, the only
thing we are doing in this functions is create a type or return a type. The return type of
`True_<T>` will always be `T` which will be known at compile-time. So whatever two values we
initially put into `True` at the end the resulting type will be known at compile-time.

With our boolean values in place, let's do some logic with the logic or and logic and. If we write
the out in lambda calculus, the implementation will also be immediately clear:
`lor = λ x . λ y . x x y`. If `x` is true, then we can return `x` knowing that it is
true, otherwise the operation can only be true if `y` is true, so we return `y`. The same goes for
the logic and: `land = λ x . λ y . x y x`. If `x` is false, we can return `x`, if it is
true the expression is true if `y` is true. Lastly to have a full logic system we're missing the
not operator, which is probably the simplest to understand: `λ x . x False True`, if it's
true, return false, else return true.

Implemented it would look something like:

``` Rust
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
```

And similar for Land and Not.

The attentive of you might have noticed that the `lor` implementation is overly complex. Since we
can just partially apply functions, we can reduce the implementation to `λ x . x x` if we so
choose. This will return a function with the first field bound and it can equally be applied to the
next function. In Haskell this is also called point-free programming.

<!-- Helper functions -->
## Helper functions

Now we want to actually get our hands on the result of the calculations we performed so far. Since
the structs / types ARE our data, we can simply print out the type name of the final value to get our
answers. Rust provides the `std::any::type_name()` function to get the string for a type. However
to get that we would have to know that type, which we explicitly don't want to know ourselves.
However we can easily build around this by creating a helper function that does the job for us:

``` Rust
fn type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

fn size_of<T>(_: &T) -> usize {
    std::mem::size_of::<T>()
}

println!("{}", type_name(&Lor.eval(True).eval(False)));
```

With this we delegate the work to actually know the type we wanna print to the compiler. We can do
the same to check the size of the type so we know how much memory it occupies.


<!-- Church numerals -->
## Church numerals

The last thing I wanna touch on are numbers. How can we represent numbers in this system? In a
system where all you can do is apply functions to other functions, numbers can be represented as a
number of function application. Zero obviously applies the function zero times. This could also be
defined as `False`, e.g. return the last parameter ignoring the first or as Zero returning the
identity function `zero = λ x . id`. Each function greater than zero is then represented as
the successor to the previous number. So one is the successor of zero and two is the successor of
one or the successor of the successor of zero.

The application of successor is then a bit more involved: `succ = λ x . λ y . λ z . y (x y z)`
we apply the function `y` once to the result of `x` applications of `y` to `z`. So each successor
applies the function once until it hits zero which does not apply the function and returns the
initial value.

Addition then can be defined as the repeated application of the successor function to a value:
`add = λ x . λ y . x succ y`. So we are basically adding one `x` times to `y` which is
functionally the same as `x + y`.

Remember the size_of function from before? Now lets see how much memory our church numeral takes up
if we, for example calculate 2 + 3:

``` Rust
let result = Add.eval(Succ.eval(Succ.eval(Zero))).eval(Succ.eval(Succ.eval(Succ.eval(Zero))));
println!("{}", type_name(&result)); // Succ_<Succ_<Succ_<Succ_<Succ_<Zero>>>>>
println!("{}", size_of(&result)); // 0
```

Yes, that's right. Our church numeral does not take up any memory at all! It's all just type
information at compile-time. And the addition we have defined is just type resolution at
compile-time. If we execute `$ strings` on our final binary, we can even see the result of the type
name already lying there. So theoretically we have created the perfect program. It computes
everything in 0 seconds (not counting compile-time). However all information must also be known at
compile-time and we have no further control on runtime, so it's probably not the most flexible
program ever written.