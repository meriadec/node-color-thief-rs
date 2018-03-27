const path = require('path')

const colorThief = require('../lib/index.js')

test('should expose a set of functions', () => {
  expect(colorThief.getDominantColor).toBeDefined()
})

test('should return a result', async () => {
  const p = path.resolve(__dirname, 'images/gradient.jpg')
  const color = await colorThief.getDominantColor(p)
  expect(color).toBe('#d05e64')
})
