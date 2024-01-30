# Rust Multi-Threaded Sorting

![image](https://github.com/fraugho/Rust-Multithreaded-Sorting/assets/144178952/a773a8fa-6f1d-47fd-8f05-542ec2df52a3)

# About
This a demo of multi-threaded sorting. I was able to achieve huge performance increased from multi-threading because of first partitioning the array into chunks for sorting of each of the threads and implmenting my own inplace merge algorithm to quickly combine the sorted sections. The ideal quantity of integers for the multihreading to increase the preformance is around 20,000 integers becuase of thread overhead. That is why my algorithm defaults to the single threaded version until the quantity of integers surpass 20,000.

# Install
```cargo run --release```

Go to localhost:8080 or 127.0.0.1 and click on try demo to redirect you to the page where you can try it out.
