//
// ABANDON ALL HOPE, ALL YE WHO ENTER HERE
//
// This is just a quick-n-dirty parser to bootstrap hAST.json
// This code may be thrown away, so don't love it.
//
var fs = require('fs')
var path = require('path')

// concatenate all the .h files into one string
var hText = ''
var dir = path.resolve(__dirname, '../../../libindy/include')
fs.readdirSync(dir).forEach(function (file) {
  file = path.resolve(dir, file)

  hText += fs.readFileSync(file, 'utf8') + '\n'
})

// split it into lines and remove comments and # lines
var lines = hText
    .split('\n')
    .map(function (line) {
      return line
            .replace(/\s+/g, ' ')
            .trim()
            .replace(/^#.*$/g, '')
            .replace(/\/\/.*$/g, '')
            .replace(/\s+/g, ' ')
            .trim()
    })
    .filter(function (line) {
      return line.length > 0
    })

// extract all inside the `extern "C" { (.*) }`
var externCText = ''
var line
var i = 0
while (i < lines.length) {
  line = lines[i]
  i++
  if (line === 'extern "C" {') {
    line = lines[i]
    while (line !== '}' && i < lines.length) {
      externCText += line + '\n'
      i++
      line = lines[i]
    }
  }
}

// extern functions on their own lines
var functionLines = (' ' + externCText.replace(/\n/g, ' ').replace(/\s+/g, ' '))
    .split('extern')
    .map(function (line) {
      return line.replace(/\s+/g, ' ').trim()
    })
    .filter(function (line) {
      return line.length > 0
    })

// parse a function args string until it hits the closing ")"
var parseArgs = function (src) {
  var args = []
  var buff = ''

  var push = function () {
    buff = buff.trim()
    if (buff.length === 0) {
      return
    }
    if (/\(|\)/.test(buff)) {
      throw new Error('Unexpected argument buffer: ' + buff)
    }
    var i = buff.lastIndexOf(' ')
    args.push({
      name: buff.substring(i).trim(),
      type: buff.substring(0, i)
    })
    buff = ''
  }

  var c
  var i = 0
  while (i < src.length) {
    c = src[i]
    if (c === ',') {
      push()
    } else if (c === ')' && !/^ *(void|indy_error_t) *\(/i.test(buff)) {
      break
    } else {
      buff += c
    }
    var m = /^ *(void|indy_error_t) *\( *\*([^)]+)\) *\(/i.exec(buff)
    if (m) {
      args.push({
        name: m[2],
        type: 'Function',
        args: parseArgs(src.substr(i + 1)),
        returnType: m[1]
      })
      buff = ''
      break
    }
    i++
  }
  push()
  return args
}

var AST = []

functionLines.forEach(function (line) {
  var m = /^(indy_error_t) (\w+) ?\((.*\));$/.exec(line)
  if (!m) {
    throw new Error('Unexpected function line: ' + line)
  }

  AST.push({
    name: m[2],
    type: 'Function',
    args: parseArgs(m[3]),
    returnType: m[1]
  })
})

module.exports = AST
