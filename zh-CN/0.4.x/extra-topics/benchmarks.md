# 性能测试

## 性能优化

在真实的生产环境中，可以尝试几种方法来优化应用：

1. 指定**内存分配器**

	- [`mimalloc`](https://crates.io/crates/mimalloc)

	- [`snmalloc-rs`](https://crates.io/crates/snmalloc-rs)

	- [`jemallocator`](https://crates.io/crates/tikv-jemallocator)

2. 声明**静态变量**

	- [`once_cell`](https://crates.io/crates/once_cell)

	- [`lazy_static`](https://crates.io/crates/lazy_static)

3. 把一小部分数据放在**栈**上

	- [`smallvec`](https://crates.io/crates/smallvec)

	- [`tinyvec`](https://crates.io/crates/tinyvec)

4. 指定 `capacity` 去创建 **vector** 或者 **hash**，在容量内的元素不会被重新分配

5. SIMD

> 在 TFB 和 WFB 测试环境中，Viz 没有使用 1、3。

## TechEmpower Framework Benchmarks (TFB)

- [仓库](https://github.com/TechEmpower/FrameworkBenchmarks)

- [结果](https://www.techempower.com/benchmarks/#test=composite)

下面是 [dc15ab19-0139-4489-b767-36eab8443eb1](https://www.techempower.com/benchmarks/#section=test&runid=dc15ab19-0139-4489-b767-36eab8443eb1&test=composite) 的结果：

![TFB dc15ab19-0139-4489-b767-36eab8443eb1](../images/TFB-dc15ab19-0139-4489-b767-36eab8443eb1.png)

## Web Frameworks Benchmark (WFB)

- [仓库](https://github.com/the-benchmarker/website)

- [结果](https://web-frameworks-benchmark.netlify.app/compare?f=actix,axum,poem,salvo,tide,viz,warp)

![WFB](../images/WFB-2023-01-09-aa5d158.png)
