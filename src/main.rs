fn main() {
    let data = vec![1, 2, 3];
    let mut list = data
        .iter()
        .map(|x| x + 1);

    let mut collected_items = vec![];

    while let Some(x) = list.next() {
        collected_items.push(x);
    }

    println!("{:?}", collected_items);
}
