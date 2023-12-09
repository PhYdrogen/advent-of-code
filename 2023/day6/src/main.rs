fn main() {
    //                               time dist
    // let races = vec![(53,250), (91,1330), (67,1081), (68,1025)];
    let races: Vec<(i64, i64)> = vec![(53916768,250133010811025)];
    // let races = vec![(7,9), (15,40), (30,200)];
    let mut result: Vec<i32> = vec![];

    for (t, dst) in &races {
        let mut count = 0;
        for course in 1..*t {
            let mult = *t - course;
            if mult * course > *dst {
                count += 1;
            }
        }
        result.push(count);
    }
    let f = result.clone();
    println!("{:?}", f.into_iter().reduce(|acc, e| acc * e));
}
