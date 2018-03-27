# node-color-thief-rs

Node.js bindings for color-thief-rs

### Usage

```js
const colorThief = require('color-thief')

const imgPath = '/tmp/sample.png'

colorThief.getDominantColor(imgPath)
  .then(color => console.log(color))
```

### Benchmark

```
rust color thief x 186 ops/sec ±0.75% (85 runs sampled)
node color thief x 62.46 ops/sec ±2.83% (67 runs sampled)
Fastest is rust color thief
```
