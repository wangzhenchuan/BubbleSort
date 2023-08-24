fn bubble_sort<T: PartialOrd>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

fn main() {
    let mut arr = [4, 2, 9, 5, 1];
    bubble_sort(&mut arr);
    println!("{:?}", arr);

    let mut arr2 = ["banana", "apple", "orange", "pineapple"];
    bubble_sort(&mut arr2);
    println!("{:?}", arr2);
}