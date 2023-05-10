fn main() {
    let random_arr = [100, 76, 10, 35, 8, 7, 92, 42, 68, 27];

    println!("Bubble Sort: {:?}", bubble_sort(random_arr));
}

fn bubble_sort(mut unsorted: [u8; 10]) -> [u8; 10]{
    let mut sorted = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    for i in (0..10).rev(){
        let mut biggest = 0;
        let mut biggest_index = 0;

        for j in 0..10{
            if unsorted[j] > biggest {
                biggest = unsorted[j];
                biggest_index = j;
            }
        }
        
        sorted[i] = biggest;
        unsorted[biggest_index] = u8::MIN;
    }

    sorted
}
