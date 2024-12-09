<template>
  <div class="container">
    <h1>Performance Comparison</h1>
    
    <div class="canvas-container">
      <div class="canvas-wrapper">
        <h2>WebAssembly Canvas (100k circles)</h2>
        <canvas id="wasm-canvas" width="800" height="600"></canvas>
        <p>Time: {{ wasmTime }}ms</p>
      </div>
      
      <div class="canvas-wrapper">
        <h2>JavaScript Canvas (100k circles)</h2>
        <canvas id="js-canvas" width="800" height="600"></canvas>
        <p>Time: {{ jsTime }}ms</p>
      </div>
    </div>

    <div class="fibonacci-container">
      <h2>Fibonacci(45) Calculation</h2>
      <p>WebAssembly Time: {{ wasmFibTime }}ms</p>
      <p>JavaScript Time: {{ jsFibTime }}ms</p>
    </div>

    <div class="controls">
      <button @click="runTests" :disabled="isRunning">{{ isRunning ? 'Running...' : 'Run Tests' }}</button>
      <div v-if="isRunning" class="progress">Running test: {{ currentTest }}</div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import init, { draw_circles, fb_wasm } from '../pkg/canvas_circle_random'

const wasmTime = ref(0)
const jsTime = ref(0)
const wasmFibTime = ref(0)
const jsFibTime = ref(0)
const isRunning = ref(false)
const currentTest = ref('')

const drawJsCircle = () => {
  const canvas = document.getElementById('js-canvas') as HTMLCanvasElement
  const ctx = canvas.getContext('2d') as CanvasRenderingContext2D
  for (let i = 0; i < 100000; i++) {
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

const sleep = (ms: number) => new Promise(resolve => setTimeout(resolve, ms))

const runTests = async () => {
  try {
    isRunning.value = true
    console.log('Starting performance tests...')
    
    // Clear canvases
    const wasmCanvas = document.getElementById('wasm-canvas') as HTMLCanvasElement
    const jsCanvas = document.getElementById('js-canvas') as HTMLCanvasElement
    
    if (!wasmCanvas || !jsCanvas) {
      throw new Error('Canvas elements not found')
    }
    
    const wasmCtx = wasmCanvas.getContext('2d')
    const jsCtx = jsCanvas.getContext('2d')
    
    if (!wasmCtx || !jsCtx) {
      throw new Error('Failed to get canvas context')
    }
    
    wasmCtx.clearRect(0, 0, wasmCanvas.width, wasmCanvas.height)
    jsCtx.clearRect(0, 0, jsCanvas.width, jsCanvas.height)

    // Add small delay between tests
    await sleep(100)

    // Test WebAssembly canvas
    currentTest.value = 'WebAssembly Canvas'
    console.log('Running WebAssembly canvas test...')
    const wasmStart = performance.now()
    draw_circles(wasmCanvas)
    wasmTime.value = Math.round(performance.now() - wasmStart)
    console.log('WebAssembly canvas test completed:', wasmTime.value, 'ms')

    await sleep(100)

    // Test JavaScript canvas
    currentTest.value = 'JavaScript Canvas'
    console.log('Running JavaScript canvas test...')
    const jsStart = performance.now()
    drawJsCircle()
    jsTime.value = Math.round(performance.now() - jsStart)
    console.log('JavaScript canvas test completed:', jsTime.value, 'ms')

    await sleep(100)

    // Test WebAssembly Fibonacci
    currentTest.value = 'WebAssembly Fibonacci(45)'
    console.log('Running WebAssembly Fibonacci test...')
    const wasmFibStart = performance.now()
    fb_wasm(45)
    wasmFibTime.value = Math.round(performance.now() - wasmFibStart)
    console.log('WebAssembly Fibonacci test completed:', wasmFibTime.value, 'ms')

    await sleep(100)

    // Test JavaScript Fibonacci
    currentTest.value = 'JavaScript Fibonacci(45)'
    console.log('Running JavaScript Fibonacci test...')
    const jsFibStart = performance.now()
    fn_js(45)
    jsFibTime.value = Math.round(performance.now() - jsFibStart)
    console.log('JavaScript Fibonacci test completed:', jsFibTime.value, 'ms')
    
    console.log('All tests completed successfully')
  } catch (error) {
    console.error('Error during tests:', error)
    alert('An error occurred during the tests. Check the console for details.')
  } finally {
    isRunning.value = false
    currentTest.value = ''
  }
}

onMounted(async () => {
  try {
    console.log('Initializing WebAssembly...')
    await init()
    console.log('WebAssembly initialized successfully')
  } catch (error) {
    console.error('Failed to initialize WebAssembly:', error)
  }
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

.controls {
  margin: 20px 0;
}

button {
  padding: 10px 20px;
  font-size: 16px;
  cursor: pointer;
}

button:disabled {
  opacity: 0.7;
  cursor: not-allowed;
}

.progress {
  margin-top: 10px;
  color: #666;
}

.fibonacci-container {
  margin: 20px 0;
}
</style>
