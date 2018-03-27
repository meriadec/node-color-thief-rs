const addon = require('../native')

exports.getDominantColor = fileName => {
  return new Promise((resolve, reject) => {
    addon.getDominantColor(fileName, color => {
      resolve(color)
    })
  })
}
