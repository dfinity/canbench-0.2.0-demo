# Canbench 0.2.0 Migration Demo

This repo shows how to migrate from [`canbench`](https://crates.io/crates/canbench) **0.1.11** to **0.2.0** using a simple benchmark example.

It’s designed to help you:
- See the **old output format** of canbench 0.1.11
- Understand the **migration process**
- Explore the **new output format** in canbench 0.2.0
- Learn about the **key improvements** introduced in the upgrade

---

## What’s in this repo

| PR | Description |
|----|-------------|
| [#2](https://github.com/dfinity/canbench-0.2.0-demo/pull/2) | Baseline benchmarks using canbench **0.1.11** |
| [#3](https://github.com/dfinity/canbench-0.2.0-demo/pull/3) | Migrates the project to canbench **0.2.0** |
| [#4](https://github.com/dfinity/canbench-0.2.0-demo/pull/4) | Small performance change to demonstrate the new output format |

---

## Why canbench 0.2.0?

[`canbench`](https://github.com/dfinity/canbench) is a benchmarking tool for Internet Computer (IC) projects.

Version **0.2.0** introduces major improvements:

- 🧼 **Cleaner output**: optional line-by-line results, now much easier to scan
- 📊 **Concise summary**: regressions, improvements, unchanged, new benchmarks
- 📥 **CSV output**: export full results to a file, also uploaded in CI
- 🔍 **New `calls` field**: tracks how many times a scope runs (useful for recursion)

📢 **Full announcement**:  
➡️ [Canbench 0.2.0 Released — Improved Output, CI Integration, and a Breaking Change](https://forum.dfinity.org/t/canbench-0-2-0-released-improved-output-ci-integration-and-a-breaking-change/50511)

---

## Try it yourself

Clone the repo, run the benchmarks, and explore the output:

```bash
git clone https://github.com/dfinity/canbench-0.2.0-demo
cd canbench-0.2.0-demo

cargo test

cd benchmarks/my_binary_heap
canbench --hide-results --show-summary --csv
```

Use `--hide-results`, `--show-summary`, and `--csv` to try out the new format.

## Tips

Always install & run with `--locked` to avoid noise from dependency updates.

If your CI compares `main` vs PR branch: expect a one-time diff when migrating from 0.1.x to 0.2.x.

## Feedback welcome

If something’s unclear or you’d like to see other examples, feel free to open an issue or discussion.

Happy benchmarking!
