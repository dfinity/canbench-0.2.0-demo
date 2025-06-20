use canbench_demo::MyBinaryHeap;
use canbench_rs::{bench, bench_fn};
use tiny_rng::{Rand, Rng};

fn generate_data(n: usize) -> Vec<u32> {
    let mut rng = Rng::from_seed(0);
    (0..n).map(|_| rng.rand_u32()).collect()
}

fn bench_new(_: usize) {
    bench_fn(|| {
        let _heap: MyBinaryHeap<u32> = MyBinaryHeap::new();
    });
}

fn bench_with_capacity(size: usize) {
    bench_fn(|| {
        let _heap: MyBinaryHeap<u32> = MyBinaryHeap::with_capacity(size);
    });
}

fn bench_push(size: usize) {
    let data = generate_data(size);
    bench_fn(|| {
        let mut heap = MyBinaryHeap::new();
        for x in &data {
            heap.push(*x);
        }
    });
}

fn bench_pop(size: usize) {
    let data = generate_data(size);
    let mut heap = MyBinaryHeap::new();
    for x in &data {
        heap.push(*x);
    }
    bench_fn(|| {
        while !heap.is_empty() {
            heap.pop();
        }
    });
}

fn bench_peek(size: usize) {
    let data = generate_data(size);
    let mut heap = MyBinaryHeap::new();
    for x in &data {
        heap.push(*x);
    }
    bench_fn(|| {
        heap.peek();
    });
}

fn bench_len(size: usize) {
    let data = generate_data(size);
    let mut heap = MyBinaryHeap::new();
    for x in &data {
        heap.push(*x);
    }
    bench_fn(|| {
        heap.len();
    });
}

fn bench_is_empty(size: usize) {
    let data = generate_data(size);
    let mut heap = MyBinaryHeap::new();
    for x in &data {
        heap.push(*x);
    }
    bench_fn(|| {
        heap.is_empty();
    });
}

fn bench_mixed(size: usize) {
    let data = generate_data(size);
    bench_fn(|| {
        let mut heap = MyBinaryHeap::new();
        for (i, x) in data.iter().enumerate() {
            if i % 4 == 0 {
                heap.pop();
            } else {
                heap.push(*x);
            }
        }
    });
}

macro_rules! bench_named {
    ($name:ident, $func:ident, $size:expr) => {
        #[bench]
        fn $name() {
            $func($size);
        }
    };
}

bench_named!(bench_heap_new_1k, bench_new, 1_000);
bench_named!(bench_heap_with_capacity_1k, bench_with_capacity, 1_000);
bench_named!(bench_push_1k, bench_push, 1_000);
bench_named!(bench_pop_1k, bench_pop, 1_000);
bench_named!(bench_peek_1k, bench_peek, 1_000);
bench_named!(bench_len_1k, bench_len, 1_000);
bench_named!(bench_is_empty_1k, bench_is_empty, 1_000);
bench_named!(bench_mixed_1k, bench_mixed, 1_000);

bench_named!(bench_heap_new_2k, bench_new, 2_000);
bench_named!(bench_heap_with_capacity_2k, bench_with_capacity, 2_000);
bench_named!(bench_push_2k, bench_push, 2_000);
bench_named!(bench_pop_2k, bench_pop, 2_000);
bench_named!(bench_peek_2k, bench_peek, 2_000);
bench_named!(bench_len_2k, bench_len, 2_000);
bench_named!(bench_is_empty_2k, bench_is_empty, 2_000);
bench_named!(bench_mixed_2k, bench_mixed, 2_000);

bench_named!(bench_heap_new_5k, bench_new, 5_000);
bench_named!(bench_heap_with_capacity_5k, bench_with_capacity, 5_000);
bench_named!(bench_push_5k, bench_push, 5_000);
bench_named!(bench_pop_5k, bench_pop, 5_000);
bench_named!(bench_peek_5k, bench_peek, 5_000);
bench_named!(bench_len_5k, bench_len, 5_000);
bench_named!(bench_is_empty_5k, bench_is_empty, 5_000);
bench_named!(bench_mixed_5k, bench_mixed, 5_000);

bench_named!(bench_heap_new_10k, bench_new, 10_000);
bench_named!(bench_heap_with_capacity_10k, bench_with_capacity, 10_000);
bench_named!(bench_push_10k, bench_push, 10_000);
bench_named!(bench_pop_10k, bench_pop, 10_000);
bench_named!(bench_peek_10k, bench_peek, 10_000);
bench_named!(bench_len_10k, bench_len, 10_000);
bench_named!(bench_is_empty_10k, bench_is_empty, 10_000);
bench_named!(bench_mixed_10k, bench_mixed, 10_000);

bench_named!(bench_heap_new_20k, bench_new, 20_000);
bench_named!(bench_heap_with_capacity_20k, bench_with_capacity, 20_000);
bench_named!(bench_push_20k, bench_push, 20_000);
bench_named!(bench_pop_20k, bench_pop, 20_000);
bench_named!(bench_peek_20k, bench_peek, 20_000);
bench_named!(bench_len_20k, bench_len, 20_000);
bench_named!(bench_is_empty_20k, bench_is_empty, 20_000);
bench_named!(bench_mixed_20k, bench_mixed, 20_000);

bench_named!(bench_heap_new_50k, bench_new, 50_000);
bench_named!(bench_heap_with_capacity_50k, bench_with_capacity, 50_000);
bench_named!(bench_push_50k, bench_push, 50_000);
bench_named!(bench_pop_50k, bench_pop, 50_000);
bench_named!(bench_peek_50k, bench_peek, 50_000);
bench_named!(bench_len_50k, bench_len, 50_000);
bench_named!(bench_is_empty_50k, bench_is_empty, 50_000);
bench_named!(bench_mixed_50k, bench_mixed, 50_000);

bench_named!(bench_heap_new_100k, bench_new, 100_000);
bench_named!(bench_heap_with_capacity_100k, bench_with_capacity, 100_000);
bench_named!(bench_push_100k, bench_push, 100_000);
bench_named!(bench_pop_100k, bench_pop, 100_000);
bench_named!(bench_peek_100k, bench_peek, 100_000);
bench_named!(bench_len_100k, bench_len, 100_000);
bench_named!(bench_is_empty_100k, bench_is_empty, 100_000);
bench_named!(bench_mixed_100k, bench_mixed, 100_000);

fn main() {}
