# Parallelization
* Parallelization is running multiple things at the same time.
* 并行化是同时运行多个事物

# Concurrency
* Concurrency is handling multiple things at the same time.
* 并发是同时处理多个事情。

# Asynchronous
* 异步不适用于处理昂贵的操作。它仅对数据来自比`RAM`更远的地方并且数据量很大的`IO`有利。异步是为并发而设计的。
* 并行化对于计算成本较高的操作是有益的。

## Async Functions
* 异步函数返回的是`Future`，它是一个函数，它返回的`Poll`。
* `Poll`有两个变量，一个表示最终的值，一个表示仍处于`pending`状态。
* `Future`是惰性的，有两种方式来运行它：
  * 一种是`tokio::spawn`出线程后，用`JoinHandle`
  * 一种是`.await`

