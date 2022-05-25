// sorting.rs

//bubble sort
pub fn bubble_sort(vec: &mut Vec<i64>) {
    // change the actual order of the array
    let mut flag = true;
    while flag == true {
        flag = false;
        // loop through, swap if vec[i] > vec[i+1]
        for i in 0..vec.capacity() - 1 {
            if vec[i] > vec[i+1] {
                // array isn't sorted forsure
                flag = true;

                // swap i and i+1
                let temp = vec[i+1];
                vec[i+1] = vec[i];
                vec[i] = temp;
            }
        }
    }
}
