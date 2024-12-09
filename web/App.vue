<template>
  <div class="container">
    <h1>Performance Comparison</h1>
    
    <div class="canvas-container">
      <div class="canvas-wrapper">
        <h2>WebAssembly Canvas</h2>
        <canvas id="wasm-canvas" width="800" height="600"></canvas>
        <p>Time: {{ wasmTime }}ms</p>
      </div>
      
      <div class="canvas-wrapper">
        <h2>JavaScript Canvas</h2>
        <canvas id="js-canvas" width="800" height="600"></canvas>
        <p>Time: {{ jsTime }}ms</p>
      </div>
    </div>

    <div class="fibonacci-container">
      <h2>Fibonacci(42) Calculation</h2>
      <p>WebAssembly Time: {{ wasmFibTime }}ms</p>
      <p>JavaScript Time: {{ jsFibTime }}ms</p>
    </div>

    <button @click="runTests">Run Tests</button>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import init, { draw_circles, fb_wasm } from '../pkg/canvas_circle_random'

const wasmTime = ref(0)
const jsTime = ref(0)
const wasmFibTime = ref(0)
const jsFibTime = ref(0)

const drawJsCircle = () => {
  const canvas = document.getElementById('js-canvas') as HTMLCanvasElement
  const ctx = canvas.getContext('2d') as CanvasRenderingContext2D
  for (let i = 0; i < 1000000; i++) {
    drawRandomCircle(ctx, 800, 600)
  }
}

const drawRandomCircle = (ctx: CanvasRenderingContext2D, width: number, height: number) => {
  const radius = 10
  const x = Math.random() * (width - 2 * radius) + radius
  const y = Math.random() * (height - 2 * radius) + radius
  const color = `rgba(${Math.floor(Math.random() * 256)}, ${Math.floor(Math.random() * 256)}, ${Math.floor(Math.random() * 256)}, ${Math.random().toFixed(2)})`
  ctx.beginPath()
  ctx.arc(x, y, radius, 0, Math.PI * 2)
  ctx.fillStyle = color
  ctx.fill()
  ctx.stroke()
}

const fn_js = (n: number): number => {
  if (n <= 1) {
    return 1
  } else {
    return fn_js(n - 1) + fn_js(n - 2)
  }
}

const runTests = async () => {
  // Clear canvases
  const wasmCanvas = document.getElementById('wasm-canvas') as HTMLCanvasElement
  const jsCanvas = document.getElementById('js-canvas') as HTMLCanvasElement
  const wasmCtx = wasmCanvas.getContext('2d')
  const jsCtx = jsCanvas.getContext('2d')
  wasmCtx?.clearRect(0, 0, wasmCanvas.width, wasmCanvas.height)
  jsCtx?.clearRect(0, 0, jsCanvas.width, jsCanvas.height)

  // Test WebAssembly canvas
  const wasmStart = performance.now()
  draw_circles(wasmCanvas)
  wasmTime.value = Math.round(performance.now() - wasmStart)

  // Test JavaScript canvas
  const jsStart = performance.now()
  drawJsCircle()
  jsTime.value = Math.round(performance.now() - jsStart)

  // Test WebAssembly Fibonacci
  const wasmFibStart = performance.now()
  fb_wasm(42)
  wasmFibTime.value = Math.round(performance.now() - wasmFibStart)

  // Test JavaScript Fibonacci
  const jsFibStart = performance.now()
  fn_js(42)
  jsFibTime.value = Math.round(performance.now() - jsFibStart)
}

onMounted(async () => {
  await init()
})
</script>

<style scoped>
.container {
  max-width: 1200px;
  margin: 0 auto;
  padding: 20px;
}

.canvas-container {
  display: flex;
  gap: 20px;
  margin: 20px 0;
}

.canvas-wrapper {
  flex: 1;
}

canvas {
  border: 1px solid #ccc;
  background: white;
}

button {
  padding: 10px 20px;
  font-size: 16px;
  cursor: pointer;
}

.fibonacci-container {
  margin: 20px 0;
}
</style>
