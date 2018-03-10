//
// ABANDON ALL HOPE, ALL YE WHO ENTER HERE
//
// This is just a quick-n-dirty parser to bootstrap src/api.json
// This code may be thrown away, so don't love it.
//
var fs = require('fs')
var path = require('path')
var stringify = require('json-stringify-pretty-compact')

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

// parse a function params string until it hits the closing ")"
var parseParams = function (src) {
  var params = []
  var buff = ''

  var push = function () {
    buff = buff.trim()
    if (buff.length === 0) {
      return
    }
    if (/\(|\)/.test(buff)) {
      throw new Error('Unexpected param buffer: ' + buff)
    }
    var i = buff.lastIndexOf(' ')
    var o = {
      name: buff.substring(i).trim(),
      type: buff.substring(0, i).replace(/ \*$/, '*')
    }
    if (/json/i.test(o.name)) {
      o.json = true
    }
    params.push(o)
    buff = ''
  }

  var o
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
      o = {
        name: m[2],
        params: parseParams(src.substr(i + 1)),
        ret: m[1]
      }
      if (o.ret === 'void') {
        delete o.ret
      }
      params.push(o)
      buff = ''
      break
    }
    i++
  }
  push()
  return params
}

var api = {
  errors: {},
  functions: {}
}

functionLines.forEach(function (line) {
  var m = /^(indy_error_t) (\w+) ?\((.*\));$/.exec(line)
  if (!m) {
    throw new Error('Unexpected function line: ' + line)
  }

  api.functions[m[2]] = {
    params: parseParams(m[3]),
    ret: m[1]
  }
})

// now parse the error codes

fs.readFileSync(path.resolve(__dirname, '../../../libindy/include/indy_mod.h'), 'utf8')
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
    api.errors['c' + pair[0]] = pair[1]
  })

fs.writeFileSync(path.resolve(__dirname, '../src/api.json'), stringify(api, {maxLength: 100}), 'utf8')
