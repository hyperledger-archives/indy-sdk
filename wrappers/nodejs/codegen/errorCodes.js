var fs = require('fs')
var path = require('path')

var file = path.resolve(__dirname, '../../../libindy/include/indy_mod.h')
var hText = fs.readFileSync(file, 'utf8')

var nameByCode = {}

hText
  .split('\n')
  .map(function (line) {
    return line
      .replace(/\s+/g, ' ')
      .trim()
      .replace(/^#.*$/g, '')
      .replace(/\/\/.*$/g, '')
      .replace(/\s+/g, ' ')
      .replace(/,/g, '')
      .trim()
  })
  .filter(function (line) {
    return line.length > 0
  })
  .slice(3, -1)
  .map(function (line) {
    return line.split('=').map(part => part.trim()).reverse()
  })
  .forEach(function (pair) {
    nameByCode['c' + pair[0]] = pair[1]
  })

console.log(JSON.stringify(nameByCode, null, 2))
