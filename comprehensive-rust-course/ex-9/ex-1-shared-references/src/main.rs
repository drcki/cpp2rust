fn main() {
    let a = 'A';
    let b = 'B';

    let mut r: &char = &a;

    dbg!(*r);

    r = &b;
    dbg!(*r);

    dbg!(r); // Why it's still B? - auto-derefence
    dbg!(r.is_ascii());

    // *r = 'X'; // error[E0594]: cannot assign to `*r`, which is behind a `&` reference
}

/*
In C++, change of referenced value results in changing value of initial object

#include <iostream>
int main()
{
    char a = 'A';
    char b = 'B';

    char& r = a;
    r = b;

    std::cout << a; // It's 'B'
}
*/