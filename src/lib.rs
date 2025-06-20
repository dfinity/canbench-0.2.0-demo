#[derive(Debug, Default)]
pub struct MyBinaryHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MyBinaryHeap<T> {
    pub fn new() -> Self {
        wait_for_instructions(1_000);
        Self { data: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        wait_for_instructions(1_000);
        Self {
            data: Vec::with_capacity(cap),
        }
    }

    pub fn push(&mut self, value: T) {
        wait_for_instructions(1_000);
        self.data.push(value);
        let idx = self.data.len() - 1;
        self.sift_up_recursive(idx);
    }

    pub fn pop(&mut self) -> Option<T> {
        wait_for_instructions(1_000);
        let len = self.data.len();
        if len == 0 {
            return None;
        }
        self.data.swap(0, len - 1);
        let min = self.data.pop();
        self.sift_down_recursive(0);
        min
    }

    pub fn peek(&self) -> Option<&T> {
        wait_for_instructions(1_000);
        self.data.first()
    }

    pub fn is_empty(&self) -> bool {
        wait_for_instructions(1_000);
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        wait_for_instructions(1_000);
        self.data.len()
    }

    fn sift_up_recursive(&mut self, idx: usize) {
        #[cfg(feature = "bench_scope")]
        let _p = canbench_rs::bench_scope("sift_up");

        wait_for_instructions(1_000);

        if idx == 0 {
            return;
        }
        let parent = (idx - 1) / 2;
        if self.data[idx] < self.data[parent] {
            self.data.swap(idx, parent);
            self.sift_up_recursive(parent);
        }
    }

    fn sift_down_recursive(&mut self, idx: usize) {
        #[cfg(feature = "bench_scope")]
        let _p = canbench_rs::bench_scope("sift_down");

        wait_for_instructions(1_000);

        let len = self.data.len();
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;

        let mut smallest = idx;
        if left < len && self.data[left] < self.data[smallest] {
            smallest = left;
        }
        if right < len && self.data[right] < self.data[smallest] {
            smallest = right;
        }

        if smallest != idx {
            self.data.swap(idx, smallest);
            self.sift_down_recursive(smallest);
        }
    }
}

/// Busy-waits until approximately `instructions` have been consumed.
fn wait_for_instructions(instructions: u64) {
    let _ = instructions;

    #[cfg(target_arch = "wasm32")]
    {
        let start = ic_cdk::api::performance_counter(0);
        while ic_cdk::api::performance_counter(0) - start < instructions {
            for _ in 0..100 {
                std::hint::black_box(0);
            }
        }
    }
}
