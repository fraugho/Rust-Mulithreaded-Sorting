use crossbeam::thread;

pub unsafe fn merge_in_place<T: PartialOrd + Copy>(
    arr: *mut T,
    start1: usize,
    end1: usize,
    start2: usize,
    end2: usize,
) {
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

pub fn multi_threaded_sorting<T: PartialOrd + std::marker::Send + Copy>(
    sort: fn(&mut [T]),
    arr: &mut [T],
) {
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
        })
        .unwrap();
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