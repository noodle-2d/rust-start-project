pub fn sort(mut vector: Vec<i32>) -> Vec<i32> {
    let vector_length = vector.len();
    quick_sort(&mut vector, 0, vector_length - 1);
    vector
}

fn quick_sort(vector: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let partition_index = partition(vector, low, high);
        if partition_index > 0 {
            quick_sort(vector, low, partition_index - 1);
        }
        quick_sort(vector, partition_index + 1, high);
    }
}

fn partition(vector: &mut Vec<i32>, low: usize, high: usize) -> usize {
    let pivot = vector[high];
    let mut i = low;

    for j in low..high {
        if vector[j] <= pivot {
            swap_elements(vector, i, j);
            i += 1;
        }
    }

    swap_elements(vector, i, high);
    i
}

fn swap_elements(vector: &mut Vec<i32>, first: usize, second: usize) {
    let swap_temp = vector[first];
    vector[first] = vector[second];
    vector[second] = swap_temp;
}
