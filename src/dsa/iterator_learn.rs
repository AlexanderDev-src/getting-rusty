pub fn iterator_fn() {
    //from C++
    //
    //vector<int>::iterator v = v1.begin() + 3

    let v1 = vec![1, -3, 4, 3, 6, -9];

    let v = &v1[3..];
}

static V1: &[i32] = &[0, 10, 20, 30, 40, 50, 60, 70, 80];
static V2: &[f32] = &[0.2f32, -4.0, 0.13, 3.15, 2.73];
pub fn iterator_interate() {
    println!("-----V1-----");
    for (idx, val) in V1.iter().enumerate() {
        println!("{} {}", idx, val)
    }
    println!("-----V2-----");
    for (idx, val) in V1.iter().enumerate() {
        println!("{} {}", idx, val)
    }

    //This is Rust idiomatic
    // auto it = v.begin() = .iter().enumerate()
    // it - v.begin() = idx from enumerate
    // *it = val from enumerate
    // it++ = automatic from enumerate
    //
    // and enumerate() = zip(range(len(v)), v) in python
    //
    //
    // This original code from c++
    //
    // auto it = v1.begin()
    // while (it < v1.end()){
    //  cout << it - v1.begin << ":" << *it << endl;
    //  it++;
    // }
}
pub fn max_min_element() {
    //from c++
    //
    //let max = max_element(V1.begin(),V1.end());
    //
    println!("MAX and Min (INT)");

    let max = V1.iter().max();
    let min = V1.iter().min();
    println!("{} {}", max.unwrap(), min.unwrap());

    println!("--------------------");
    println!("MAX and Min (FLOAT)");
    //from c++
    //
    //let max = max_element(V2.begin()+3, V2.end());
    let max_2 = V2[3..].iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let min_2 = V2[3..].iter().cloned().fold(f32::INFINITY, f32::min);
    println!("{} {}", max_2, min_2);
}
pub fn iterator_invaild() {
    let mut s1 = vec![
        String::from("somchai"),
        String::from("somying"),
        String::from("somsak"),
    ];

    for x in &s1 {
        print!("{}, ", x);
    }

    println!();

    //edit in-place
    for x in &mut s1 {
        x.replace_range(0..4, "--");
    }

    for x in &s1 {
        print!("{}, ", x);
    }

    println!();

    //from c++
    //
    //for(string x: s1){
    // cout << x << ", ";
    //}
    //
    //for(auto x: s1){x.replace(0,4,"--");}
    //for(auto x: s1){cout << x << ", ";}
}
