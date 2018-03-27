const colorThief = require('../lib/index.js')

test('should expose a set of functions', () => {
  expect(colorThief.getDominantColor).toBeDefined()
})

test('should return a result', () => {
  const res = colorThief.getDominantColor('/toto/titi.png')
  expect(res).toBeDefined()
})
