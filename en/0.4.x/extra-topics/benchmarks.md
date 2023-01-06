# Benchmarks

## Performance Optimization

In a real production environment, several approaches can be tried to optimize the application:

1. Specify **memory allocators**

2. Declaring **static variables**

3. Putting a small portion of data on the **stack**

> In the TFB and WFB test environments, Viz did not use 1 and 3.

## TechEmpower Framework Benchmarks (TFB)

- [Repository](https://github.com/TechEmpower/FrameworkBenchmarks)

- [Result](https://www.techempower.com/benchmarks/#test=composite)

Here are the results for [7ce481b2-49ec-4a4d-952d-bb1334](https://www.techempower.com/benchmarks/#section=test&runid=7ce481b2-49ec-4a4d-952d-bb1334d4a4ad&test=composite):

![TFB 7ce481b2-49ec-4a4d-952d-bb1334d4a4ad](../images/TFB-7ce481b2-49ec-4a4d-952d-bb1334d4a4ad.png)

## Web Frameworks Benchmark (WFB)

- [Repository](https://github.com/the-benchmarker/website)

- [Result](https://web-frameworks-benchmark.netlify.app/compare?f=actix,axum,poem,salvo,tide,viz,warp)

![WFB](../images/WFB-2023-01-01-0b5c28a.png)
