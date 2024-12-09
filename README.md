# WebAssembly vs JavaScript Performance Comparison

A small experiment comparing the performance of WebAssembly, JavaScript, and native Rust in two scenarios.

## Test Scenarios

### 1. Circle Drawing
Randomly generate 100,000 colored circles
- Native Rust: ~64ms
- WebAssembly: 678ms
- JavaScript: 425ms

### 2. Fibonacci Calculation
Calculate Fibonacci(45)
- Native Rust: ~4.7s
- WebAssembly: 3.75s
- JavaScript: 18s

## Conclusions

1. For scenarios requiring frequent JS/Web API calls, plain JavaScript might perform better
2. For computation-intensive applications (like image processing or physics engines), WebAssembly offers significant performance improvements
3. While native implementation provides the best performance, WebAssembly is still considerably faster than JavaScript

---

# WebAssembly vs JavaScript 性能对比

小实验，对比了 WebAssembly、JavaScript 和原生 Rust 在两个场景下的性能表现。

## 实验场景

### 1. 画圈
随机生成 10 万个彩色小圆圈
- Native Rust: ~64ms
- WebAssembly: 678ms
- JavaScript: 425ms

### 2. 斐波那契数列计算 
计算 Fibonacci(45)
- Native Rust: ~4.7s
- WebAssembly: 3.75s
- JavaScript: 18s

## 总结

1. 在需要频繁调用 JS/Web API 的场景下，可能还不如直接用 JS
2. 如果你的应用涉及大量计算（比如图像处理、物理引擎之类的），WASM 确实能带来显著的性能提升
3. 想要最极致的性能还得用原生实现，但 WASM 也不赖，至少比 JS 强多了
