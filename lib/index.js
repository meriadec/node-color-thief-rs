const addon = require('../native')

exports.getDominantColor = fileName => {
  return addon.getDominantColor(fileName)
}
