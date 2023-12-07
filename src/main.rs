//use std::thread;
use rand::Rng;
use std::time::Instant;
use crossbeam::thread;
use charts::{Chart, VerticalBarView, ScaleBand, ScaleLinear};


unsafe fn merge_in_place(arr: *mut i32, start1: usize, end1: usize, start2: usize, end2: usize) {
    let mut temp = Vec::with_capacity(end2 - start1);
    let mut i = start1;
    let mut j = start2;

    // Copy elements to the temporary vector
    while i < end1 && j < end2 {
        if *arr.add(i) <= *arr.add(j) {
            temp.push(*arr.add(i));
            i += 1;
        } else {
            temp.push(*arr.add(j));
            j += 1;
        }
    }

    // Copy any remaining elements from the first sub-array
    while i < end1 {
        temp.push(*arr.add(i));
        i += 1;
    }

    // Copy any remaining elements from the second sub-array
    while j < end2 {
        temp.push(*arr.add(j));
        j += 1;
    }

    // Copy back from temp to the array
    for (k, &item) in temp.iter().enumerate() {
        *arr.add(start1 + k) = item;
    }
}

fn multi_threaded_sorting(sort: fn(&mut [i32]), arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    if len > 20000 {
        let num_threads = num_cpus::get();
        let sub_array_size = len / num_threads;

        thread::scope(|s| {
            let (mut left, mut right) = arr.split_at_mut(sub_array_size);
            s.spawn(move |_| {
                sort(left);
            });
            for _ in 1..num_threads - 1 {
                let (new_left, new_right) = right.split_at_mut(sub_array_size);
                left = new_left;
                right = new_right;
                let left_to_sort = left;
                s.spawn(move |_| {
                    sort(left_to_sort);
                });
            }
            s.spawn(move |_| {
                sort(right);
            });
        }).unwrap();
            // Iterative merging
            let arr_ptr = arr.as_mut_ptr();
            let mut merge_size = sub_array_size;
            while merge_size < len {
                let mut i = 0;
                while i < len {
                    let start1 = i;
                    let end1 = std::cmp::min(i + merge_size, len);
                    let start2 = end1;
                    let end2 = std::cmp::min(start2 + merge_size, len);
    
                    if start2 < end2 {
                        unsafe {
                            merge_in_place(arr_ptr, start1, end1, start2, end2);
                        }
                    }
                    i += 2 * merge_size;
                }
                merge_size *= 2;
            }
    } else {
        sort(arr);
    }
}


fn quick_sort(arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    let pivot = partition(arr);
    quick_sort(&mut arr[0..pivot]); // Sort the left part
    quick_sort(&mut arr[pivot + 1..len]); // Sort the right part
}

fn partition(arr: &mut [i32]) -> usize {
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


fn heap_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }

    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify(arr: &mut [i32], len: usize, i: usize) {
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

/*
fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1]{ 
                arr.swap(j, j + 1);
            }
        }
    }
}
*/

fn main() {

    let mut arr = Vec::new();
    let mut arr2 = vec![];
    let mut arr3 = Vec::new();
    let mut arr4 = vec![];
    let mut arr5 = Vec::new();
    let mut arr6 = vec![];
    
    let mut rng = rand::thread_rng();
    
    for _i in 0..200000 {
        arr.push(rng.gen_range(1..=10_000));
        arr2.push(rng.gen_range(1..=10_000));
        arr3.push(rng.gen_range(1..=10_000));
        arr4.push(rng.gen_range(1..=10_000));
        arr5.push(rng.gen_range(1..=10_000));
        arr6.push(rng.gen_range(1..=10_000));
    }

    let mut start = Instant::now();
    //parallel_sort( &mut arr, bubble_sort);
    let multi_bubble_duration = start.elapsed();

    start = Instant::now();
    //bubble_sort(&mut arr2);
    let bubble_duration = start.elapsed();

    start = Instant::now();
    //parallel_sort( &mut arr3, quick_sort);
    multi_threaded_sorting(quick_sort, &mut arr3);
    let multi_quick_duration = start.elapsed();

    start = Instant::now();
    quick_sort(&mut arr4);
    let quick_duration = start.elapsed();

    start = Instant::now();
    multi_threaded_sorting(heap_sort, &mut arr5);
    //parallel_sort( &mut arr5, heap_sort);
    let multi_heap_duration = start.elapsed();

    start = Instant::now();
    heap_sort(&mut arr6);
    let heap_duration = start.elapsed();

    println!("The Multi-Threaded Bubble Sort took {:?}", multi_bubble_duration);
    println!("The Single-Threaded Bubble Sort took {:?}", bubble_duration);
    println!("The Multi-Threaded Quick Sort took {:?}", multi_quick_duration);
    println!("The Single-Threaded Quick Sort took {:?}", quick_duration);
    println!("The Multi-Threaded Heap Sort took {:?}", multi_heap_duration);
    println!("The Single-Threaded Heap Sort took {:?}", heap_duration);

    // Define chart related sizes.
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create a band scale that maps ["A", "B", "C"] categories to values in the [0, availableWidth]
    // range (the width of the chart without the margins).
    let x = ScaleBand::new()
        .set_domain(vec![String::from("Single-Threaded Quick Sort"), String::from("Multi-Threaded Quick Sort"), String::from("Single-Threaded Heap Sort"), String::from("Multi-Threaded Heap Sort")])
        .set_range(vec![0, width - left - right])
        .set_inner_padding(0.1)
        .set_outer_padding(0.1);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y = ScaleLinear::new()
        .set_domain(vec![0.0, 1000.0])
        .set_range(vec![height - top - bottom, 0]);

    // You can use your own iterable as data as long as its items implement the `BarDatum` trait.
    let data = vec![
        ("Single-Threaded Quick Sort", quick_duration.as_millis() as f32), 
        ("Multi-Threaded Quick Sort", multi_quick_duration.as_millis() as f32), 
        ("Single-Threaded Heap Sort", heap_duration.as_millis() as f32), 
        ("Multi-Threaded Heap Sort", multi_heap_duration.as_millis() as f32)
    ];
        
    // Create VerticalBar view that is going to represent the data as vertical bars.
    let view = VerticalBarView::new()
    .set_x_scale(&x)
    .set_y_scale(&y)
    .load_data(&data).unwrap();



    // Generate and save the chart.
    Chart::new()
        .set_width(width)
        .set_height(height)
        .set_margins(top, right, bottom, left)
        .add_title(String::from("Bar Chart"))
        .add_view(&view)
        .add_axis_bottom(&x)
        .add_axis_left(&y)
        .add_left_axis_label("In Milliseconds")
        .add_bottom_axis_label("Sorting Algorithms")
        .save("vertical-bar-chart2.svg").unwrap();

}

/*
fn inplace_merge<T: Ord>(arr: &mut [i32], start1: usize, end1: usize, end2: usize) {
    let mut temp = Vec::new();
    let mut i = start1;
    let mut j = end1;

    // Copy the first partition into a temporary vector
    while i < end1 {
        temp.push(arr[i].clone());
        i += 1;
    }

    let mut k = start1;
    i = 0;

    // Merge the two partitions
    while k < end2 && j < end2 {
        if temp[i] <= arr[j] {
            arr[k] = temp[i].clone();
            i += 1;
        } else {
            arr[k] = arr[j].clone();
            j += 1;
        }
        k += 1;
    }

    // Copy any remaining elements from the temporary vector
    while i < temp.len() {
        arr[k] = temp[i].clone();
        i += 1;
        k += 1;
    }
}
*/

/*
fn multi_threaded_sorting(sort: fn(&mut [i32]), arr: &mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    if len > 800 {
        let num_threads = num_cpus::get();
        let sub_array_size = len / num_threads;

        thread::scope(|s| {
            let (mut left, mut right) = arr.split_at_mut(sub_array_size);
            s.spawn(move |_| {
                sort(left);
            });
            for _ in 1..num_threads - 1 {
                let (new_left, new_right) = right.split_at_mut(sub_array_size);
                left = new_left;
                right = new_right;
                let left_to_sort = left;
                s.spawn(move |_| {
                    sort(left_to_sort);
                });
            }
            s.spawn(move |_| {
                sort(right);
            });
        }).unwrap();
    } else {
        sort(arr);
    }
    // Unsafe merging
    let num_threads = num_cpus::get();
    let arr_ptr = arr.as_mut_ptr();
    let chunk_size = len / num_threads;
    for i in 0..(num_threads - 1) {
        let start1 = i * chunk_size;
        let end1 = start1 + chunk_size;
        let start2 = end1;
        let end2 = if i == num_threads - 2 { len } else { start2 + chunk_size };

        unsafe {
            merge_in_place(arr_ptr, start1, end1, start2, end2);
        }
    }
}
*/
/*
fn parallel_sort<T, F>(arr: &mut [T], sort_fn: F)
where
    T: Send + Ord + Clone + 'static, // Add 'static lifetime bound
    F: Fn(&mut [T]) + Sync + Send + Copy + 'static, // Add 'static lifetime bound
{
    let len = arr.len();
    let num_threads = num_cpus::get(); // or use a dynamic value based on your environment
    let chunk_size = len / num_threads + if len % num_threads > 0 { 1 } else { 0 };

    let mut threads = Vec::new();

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = std::cmp::min(start + chunk_size, len);

        let mut sub_arr = arr[start..end].to_vec();
        threads.push(thread::spawn(move || {
            sort_fn(&mut sub_arr);
            sub_arr
        }));
    }

    let mut sorted_sub_arrays = Vec::new();
    for thread in threads {
        sorted_sub_arrays.push(thread.join().expect("Thread panicked"));
    }

    // Merge sorted sub-arrays back into the original array
    let merged_array = merge_sorted_arrays(sorted_sub_arrays);
    for (i, item) in merged_array.iter().enumerate() {
        arr[i] = item.clone();
    }
}
*/
/*
fn merge_sorted_arrays<T: Ord + Clone>(mut arrays: Vec<Vec<T>>) -> Vec<T> {
    let mut result = Vec::new();
    while arrays.iter().any(|arr| !arr.is_empty()) {
        let (min_index, _) = arrays
            .iter()
            .enumerate()
            .filter_map(|(i, arr)| arr.first().map(|x| (i, x)))
            .min_by_key(|&(_, item)| item)
            .unwrap();

        result.push(arrays[min_index].remove(0));
    }
    result
}
*/