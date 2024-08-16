fn main() {
    // Iter fun! based on:
    // https://ntietz.com/blog/rusts-iterators-optimize-footgun/

    // Rust's iterators are lazy: that means the iterator increments
    // but the methods/functions don't evaluate unless required.
    // For example, a condition imposed by a preceding function 
    // may not be met, in which case the iterators continues to the
    // next incremented step without evaluating anything beyond the
    // failed condition.

    let xs = [1, 2, 3, 4, 5];

    // In both composed functions below (v1 & v2), the filter 
    // is applied for each iteration, but map is applied only
    // for those iterations that satisfy the filter.

    // v1
    xs.iter()
        .filter(|x| {println!("filter {} for mod 2", x); *x % 2 == 0})
        .map(|x| {println!("map on {} to x + 1", x); x+1})
        .for_each(|x| println!("v1 result: {}", x));

    // v2
    xs.iter().filter_map(|x| if *x % 2 == 0 { Some(x + 1) } else { None })
        .for_each(|x| println!("v2 result: {}", x));

    // v3
    for x in xs.iter() {
        if *x % 2 == 0 {
            println!("v3 result: {}", x + 1);
        }
    }

    // The v1, v2, and v3 result values all line up:
    // v1 result: 3
    // v1 result: 5
    // v2 result: 3
    // v2 result: 5
    // v3 result: 3
    // v3 result: 5

}
