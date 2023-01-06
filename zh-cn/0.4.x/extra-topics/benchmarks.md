# 性能测试

## 性能优化

在真实的生产环境中，可以尝试几种方法来优化应用：

1. 指定**内存分配器**

	- [`mimalloc`](https://crates.io/crates/mimalloc)

	- [`snmalloc-rs`](https://crates.io/crates/snmalloc-rs)

2. 声明**静态变量**

	- [`once_cell`](https://crates.io/crates/once_cell)

	- [`lazy_static`](https://crates.io/crates/lazy_static)

3. 把一小部分数据放在**栈**上

	- [`smallvec`](https://crates.io/crates/smallvec)

	- [`tinyvec`](https://crates.io/crates/tinyvec)

> 在 TFB 和 WFB 测试环境中，Viz 没有使用 1、3。

## TechEmpower Framework Benchmarks (TFB)

- [仓库](https://github.com/TechEmpower/FrameworkBenchmarks)

- [结果](https://www.techempower.com/benchmarks/#test=composite)

下面是 [7ce481b2-49ec-4a4d-952d-bb1334](https://www.techempower.com/benchmarks/#section=test&runid=7ce481b2-49ec-4a4d-952d-bb1334d4a4ad&test=composite) 的结果：

![TFB 7ce481b2-49ec-4a4d-952d-bb1334d4a4ad](../images/TFB-7ce481b2-49ec-4a4d-952d-bb1334d4a4ad.png)

## Web Frameworks Benchmark (WFB)

- [仓库](https://github.com/the-benchmarker/website)

- [结果](https://web-frameworks-benchmark.netlify.app/compare?f=actix,axum,poem,salvo,tide,viz,warp)

![WFB](../images/WFB-2023-01-01-0b5c28a.png)
