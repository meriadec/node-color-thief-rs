const Benchmark = require('benchmark')
const ColorThief = require('color-thief')
const path = require('path')

const suite = new Benchmark.Suite()
const rustColorThief = require('../lib/index.js')

const p = path.resolve(__dirname, '../__tests__/images/gradient.jpg')

suite
  .add('rust color thief', async () => {
    const color = await rustColorThief.getDominantColor(p)
    return color
  })
  .add('node color thief', async () => {
    var colorThief = new ColorThief()
    const color = colorThief.getColor(p)
    return color
  })
  .on('cycle', event => {
    console.log(String(event.target))
  })
  .on('complete', function() {
    console.log('Fastest is ' + this.filter('fastest').map('name'))
  })
  .run({ async: true })
