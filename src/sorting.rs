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

pub fn selection_sort(vec: &mut Vec<i64>) {
    // put each value in it's correct position

    for i in 0..vec.capacity() {

        let mut index = i;

        for j in i..vec.capacity() {

            if vec[j] < vec[index] {
                index = j;
            }
        }

        // swap index and i values
        if index != i {
            let temp = vec[index];
            vec[index] = vec[i];
            vec[i] = temp;
        }
    }
    
}

pub fn insertion_sort(vec: &mut Vec<i64>) {
    println!("Vec capacity is {}", vec.capacity());
    for i in 0..vec.capacity() {

        let mut index = i;

        while index > 0 && vec[index] < vec[index-1] {
            // swap em 

            let temp = vec[index];
            vec[index] = vec[index-1];
            vec[index-1] = temp;

            // increment index
            index-=1;
        }
    }
}
