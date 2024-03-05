// Importing necessary crates
use charts::{Chart, ScaleBand, ScaleLinear, VerticalBarView};
use crossbeam::thread;
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Instant;

unsafe fn merge_in_place<T: PartialOrd + Copy>(
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

fn multi_threaded_sorting<T: PartialOrd + std::marker::Send + Copy>(
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

fn quick_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    let pivot = partition(arr);
    quick_sort(&mut arr[0..pivot]); // Sort the left part
    quick_sort(&mut arr[pivot + 1..len]); // Sort the right part
}

fn partition<T: PartialOrd + Copy>(arr: &mut [T]) -> usize {
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

fn heap_sort<T: PartialOrd + Copy>(arr: &mut [T]) {
    let len = arr.len();
    for i in (0..len / 2).rev() {
        heapify(arr, len, i);
    }

    for i in (1..len).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify<T: PartialOrd + Copy>(arr: &mut [T], len: usize, i: usize) {
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

pub fn svgmake(array_size: u64) -> Result<(), Box<dyn std::error::Error>> {
    // Data preparation
    let mut rng = rand::thread_rng();
    let mut arr = vec![];
    for _i in 0..array_size {
        //arr.push(rng.gen_range(1..=10_000));
        arr.push(rng.gen_range(0.0..=10000.0));
    }

    let mut start = Instant::now();
    multi_threaded_sorting(quick_sort, &mut arr);
    let multi_quick_duration = start.elapsed();

    // Reinitialize array for next sort
    arr.shuffle(&mut rng);

    start = Instant::now();
    quick_sort(&mut arr);
    let quick_duration = start.elapsed();

    arr.shuffle(&mut rng);

    start = Instant::now();
    multi_threaded_sorting(heap_sort, &mut arr);
    let multi_heap_duration = start.elapsed();

    arr.shuffle(&mut rng);

    start = Instant::now();
    heap_sort(&mut arr);
    let heap_duration = start.elapsed();

    println!(
        "The Multi-Threaded Quick Sort took {:?}",
        multi_quick_duration
    );
    println!("The Single-Threaded Quick Sort took {:?}", quick_duration);
    println!(
        "The Multi-Threaded Heap Sort took {:?}",
        multi_heap_duration
    );
    println!("The Single-Threaded Heap Sort took {:?}", heap_duration);

    // Define chart related sizes.
    let width = 800;
    let height = 600;
    let (top, right, bottom, left) = (90, 40, 50, 60);

    // Create a band scale that maps ["A", "B", "C"] categories to values in the [0, availableWidth]
    // range (the width of the chart without the margins).
    let x = ScaleBand::new()
        .set_domain(vec![
            String::from("Single-Threaded Quick Sort"),
            String::from("Multi-Threaded Quick Sort"),
            String::from("Single-Threaded Heap Sort"),
            String::from("Multi-Threaded Heap Sort"),
        ])
        .set_range(vec![0, width - left - right])
        .set_inner_padding(0.1)
        .set_outer_padding(0.1);

    // Create a linear scale that will interpolate values in [0, 100] range to corresponding
    // values in [availableHeight, 0] range (the height of the chart without the margins).
    // The [availableHeight, 0] range is inverted because SVGs coordinate system's origin is
    // in top left corner, while chart's origin is in bottom left corner, hence we need to invert
    // the range on Y axis for the chart to display as though its origin is at bottom left.
    let y;
    let data;
    let view;
    let largest = std::cmp::max(
        std::cmp::max(quick_duration, heap_duration),
        std::cmp::max(multi_quick_duration, multi_heap_duration),
    );

    if largest.as_millis() > 1000 {
        y = ScaleLinear::new()
            .set_domain(vec![0.0, (largest.as_secs() as f32 * 1.2).round()])
            .set_range(vec![height - top - bottom, 0]);

        data = vec![
            ("Single-Threaded Quick Sort", quick_duration.as_secs_f32()),
            (
                "Multi-Threaded Quick Sort",
                multi_quick_duration.as_secs_f32(),
            ),
            ("Single-Threaded Heap Sort", heap_duration.as_secs_f32()),
            (
                "Multi-Threaded Heap Sort",
                multi_heap_duration.as_secs_f32(),
            ),
        ];

        // Create VerticalBar view that is going to represent the data as vertical bars.
        view = VerticalBarView::new()
            .set_x_scale(&x)
            .set_y_scale(&y)
            .load_data(&data)
            .unwrap();

        // Generate and save the chart.
        Chart::new()
            .set_width(width)
            .set_height(height)
            .set_margins(top, right, bottom, left)
            //.add_title(String::from(""))
            .add_view(&view)
            .add_axis_bottom(&x)
            .add_axis_left(&y)
            .add_left_axis_label("In Seconds")
            .add_bottom_axis_label("Sorting Algorithms")
            .save("static/charts/vertical-bar-chart.svg")
            .unwrap();
    } else if largest.as_micros() > 1000 {
        y = ScaleLinear::new()
            .set_domain(vec![0.0, (largest.as_millis() as f32 * 1.2).round()])
            .set_range(vec![height - top - bottom, 0]);

        data = vec![
            (
                "Single-Threaded Quick Sort",
                quick_duration.as_millis() as f32,
            ),
            (
                "Multi-Threaded Quick Sort",
                multi_quick_duration.as_millis() as f32,
            ),
            (
                "Single-Threaded Heap Sort",
                heap_duration.as_millis() as f32,
            ),
            (
                "Multi-Threaded Heap Sort",
                multi_heap_duration.as_millis() as f32,
            ),
        ];

        // Create VerticalBar view that is going to represent the data as vertical bars.
        view = VerticalBarView::new()
            .set_x_scale(&x)
            .set_y_scale(&y)
            .load_data(&data)
            .unwrap();

        // Generate and save the chart.
        Chart::new()
            .set_width(width)
            .set_height(height)
            .set_margins(top, right, bottom, left)
            //.add_title(String::from(""))
            .add_view(&view)
            .add_axis_bottom(&x)
            .add_axis_left(&y)
            .add_left_axis_label("In Milliseconds")
            .add_bottom_axis_label("Sorting Algorithms")
            .save("static/charts/vertical-bar-chart.svg")
            .unwrap();
    } else if largest.as_nanos() > 1000 {
        y = ScaleLinear::new()
            .set_domain(vec![0.0, (largest.as_micros() as f32 * 1.2).round()])
            .set_range(vec![height - top - bottom, 0]);

        data = vec![
            (
                "Single-Threaded Quick Sort",
                quick_duration.as_micros() as f32,
            ),
            (
                "Multi-Threaded Quick Sort",
                multi_quick_duration.as_micros() as f32,
            ),
            (
                "Single-Threaded Heap Sort",
                heap_duration.as_micros() as f32,
            ),
            (
                "Multi-Threaded Heap Sort",
                multi_heap_duration.as_micros() as f32,
            ),
        ];

        // Create VerticalBar view that is going to represent the data as vertical bars.
        view = VerticalBarView::new()
            .set_x_scale(&x)
            .set_y_scale(&y)
            .load_data(&data)
            .unwrap();

        // Generate and save the chart.
        Chart::new()
            .set_width(width)
            .set_height(height)
            .set_margins(top, right, bottom, left)
            .add_title(String::from(""))
            .add_view(&view)
            .add_axis_bottom(&x)
            .add_axis_left(&y)
            .add_left_axis_label("In Microseconds")
            .add_bottom_axis_label("Sorting Algorithms")
            .save("static/charts/vertical-bar-chart.svg")
            .unwrap();
    } else {
        y = ScaleLinear::new()
            .set_domain(vec![0.0, (largest.as_nanos() as f32 * 1.2).round()])
            .set_range(vec![height - top - bottom, 0]);

        data = vec![
            (
                "Single-Threaded Quick Sort",
                quick_duration.as_nanos() as f32,
            ),
            (
                "Multi-Threaded Quick Sort",
                multi_quick_duration.as_nanos() as f32,
            ),
            ("Single-Threaded Heap Sort", heap_duration.as_nanos() as f32),
            (
                "Multi-Threaded Heap Sort",
                multi_heap_duration.as_nanos() as f32,
            ),
        ];

        // Create VerticalBar view that is going to represent the data as vertical bars.
        view = VerticalBarView::new()
            .set_x_scale(&x)
            .set_y_scale(&y)
            .load_data(&data)
            .unwrap();

        // Generate and save the chart.
        Chart::new()
            .set_width(width)
            .set_height(height)
            .set_margins(top, right, bottom, left)
            //.add_title(String::from(""))
            .add_view(&view)
            .add_axis_bottom(&x)
            .add_axis_left(&y)
            .add_left_axis_label("In Nanoseconds")
            .add_bottom_axis_label("Sorting Algorithms")
            .save("static/charts/vertical-bar-chart.svg")
            .unwrap();
    }
    Ok(())
}
