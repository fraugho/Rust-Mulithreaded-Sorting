use crate::multi_threading::*;
use crate::sorting::*;
use charts::{Chart, ScaleBand, ScaleLinear, VerticalBarView};
use rand::seq::SliceRandom;
use rand::Rng;
use std::time::Instant;

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
