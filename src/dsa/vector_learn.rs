// What is Vector?
//
// [1][2][3]?
//

pub fn vector_learn_fn() {
    // This is wrong!
    //let mut x: Vec<i32>;

    //for i in 0..=7 {
    //    x[i] = i;
    //}
    // Do this instead
    let mut x: Vec<i32> = Vec::new();

    for i in 0..=7 {
        x.push(i);
    }

    println!("{:?}", x);
}
pub fn find_vector_fn() {
    let x = vec![1, 2, 3, 5, 6, 9];

    let item = x.iter().find(|&&x| x == 5);
    print!("{:?}", item);

    // Why |&&x| ?
    // The first one is from .find() in function find the parameter use &Self::Item
    // and the second one is from iter()
    //
    // so let see this
    //
    let index = x.iter().position(|&x| x == 5);
    print!("{:?}", index)

    // SO why find the index only use 1 & because the & only from .iter()
    // huh? so what about position??
    // in position function the parameter us Self::Item. See the difference?
}

pub fn find_adv_fn() {
    let x = vec![9, -1, 3, 7, 5, 2, 1, 4];

    let y = 5;

    if x.get(..4).map_or(false, |sub| sub.contains(&y)) {
        print!("Yes");
    } else {
        print!("No");
    }
    // This if statement mean. get index 0 -> 2 but not 3.
    // .map_or mean. if Option = None use default in this if statement is false
    // so if Option = Some(something) use sub.contains(&3)
    // and sub.contains(&3) mean have number 3 inside?
}
