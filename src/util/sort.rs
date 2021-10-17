pub fn sort<T: Copy + PartialOrd>(mut vector: Vec<T>) -> Vec<T> {
    let vector_length = vector.len();
    if vector_length > 0 {
        quick_sort(&mut vector, 0, vector_length - 1);
    }
    vector
}

fn quick_sort<T: Copy + PartialOrd>(vector: &mut Vec<T>, low: usize, high: usize) {
    let partition_index = partition(vector, low, high);
    if partition_index > 0 && low < partition_index - 1 {
        quick_sort(vector, low, partition_index - 1);
    }
    if partition_index + 1 < high {
        quick_sort(vector, partition_index + 1, high);
    }
}

fn partition<T: Copy + PartialOrd>(vector: &mut Vec<T>, low: usize, high: usize) -> usize {
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

fn swap_elements<T: Copy>(vector: &mut Vec<T>, first: usize, second: usize) {
    let swap_temp = vector[first];
    vector[first] = vector[second];
    vector[second] = swap_temp;
}
