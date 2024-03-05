pub fn quick_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    let pivot = partition(arr);
    quick_sort(&mut arr[0..pivot]); // Sort the left part
    quick_sort(&mut arr[pivot + 1..len]); // Sort the right part
}

pub fn partition<T: PartialOrd + Copy>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}

pub fn heap_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let len = arr.len();
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }

    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

pub fn heapify<T: PartialOrd + Copy>(arr: &mut [T], len: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < len && arr[left] > arr[largest] {
        largest = left;
    }

    if right < len && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        heapify(arr, len, largest);
    }
}