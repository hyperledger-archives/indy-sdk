var fs = require('fs')
var path = require('path')
var apiFunctions = require('./apiFunctions')
var mdEscape = require('markdown-escape')

var README_FILE = path.resolve(__dirname, '../README.md')

var toHumanType = function (param) {
  if (param.json) {
    return 'Json'
  }
  if (param.timestamp) {
    return 'Timestamp (Number)'
  }
  switch (param.type.replace(/[^a-z0-9_*]/ig, '')) {
    case 'constchar*':
    case 'constchar*const':
      return 'String'

    case 'indy_bool_t':
      return 'Boolean'

    case 'indy_error_t':
      return 'IndyError'

    case 'indy_handle_t':
      return 'Handle (Number)'

    case 'indy_u32_t':
    case 'indy_i32_t':
    case 'indy_u64_t':
      return 'Number'

    case 'Buffer':
      return 'Buffer'
  }
  throw new Error('toHumanType doesn\'t handle: ' + param.type)
}

var readmeParam = function (param) {
  return '`' + param.jsName + '`: ' + toHumanType(param)
}

var apiFunctionsGrouped = {}
apiFunctions.forEach(function (fn) {
  if (!apiFunctionsGrouped[fn.group]) {
    apiFunctionsGrouped[fn.group] = []
  }
  apiFunctionsGrouped[fn.group].push(fn)
})

var readme = ''

Object.keys(apiFunctionsGrouped).forEach(function (group) {
  readme += '### ' + group + '\n\n'
  apiFunctionsGrouped[group].forEach(readmeFn)
})

function markdownify (src) {
  var lines = src.split('\n')
  var out = []
  var line
  var i = 0
  while (i < lines.length) {
    line = lines[i]
    i++
    if (line.trim() === '{' || line.trim() === '[{') {
      out.push('```')
      out.push(line)
      while (i < lines.length) {
        line = lines[i]
        i++
        out.push(line)
      }
      out.push('````')
    } else if (/^\s*\*/.test(line)) {
      var parts = line.split('*')
      out.push(parts[0] + '* ' + mdEscape(parts.slice(1).join('*')))
    } else {
      line = mdEscape(line)
        .replace(/\s+/, ' ')
        .trim()
      out.push(line)
    }
  }
  return out.join('\n')
}

function readmeFn (fn) {
  var docAST = parseDocString(fn.docs)

  var signature = fn.jsName + ' ( ' + fn.jsParams.map(arg => arg.jsName).join(', ') + ' ) -> ' + fn.humanReturnValue
  readme += '#### ' + markdownify(signature) + '\n\n'

  readme += markdownify(docAST.desc) + '\n\n'

  fn.jsParams.forEach(function (arg) {
    readme += '* ' + readmeParam(arg)
    if (docAST.params[arg.name]) {
      if (docAST.params[arg.name].optional) {
        readme += '?'
      }
      if (docAST.params[arg.name].text.trim().length > 0) {
        if (arg.jsName === 'wh') {
          readme += ' - wallet handle (created by openWallet)'
        } else {
          readme += ' - ' + markdownify(docAST.params[arg.name].text)
        }
      }
    }
    readme += '\n'
  })
  readme += '* __->__ '
  if (fn.jsCbParams.length === 0) {
    readme += 'void'
  } else if (fn.jsCbParams.length === 1) {
    readme += readmeParam(fn.jsCbParams[0])
  } else if (fn.jsCbParams.length > 1) {
    readme += '[ ' + fn.jsCbParams.map(readmeParam).join(', ') + ' ]'
  }
  if (fn.jsCbParams.length > 0 && docAST.returns.length > 0) {
    readme += ' - ' + markdownify(docAST.returns)
  }
  readme += '\n'
  readme += '\n'
  if (docAST.errors.length > 0) {
    readme += 'Errors: `' + docAST.errors.join('`, `') + '`\n'
  }
  readme += '\n'
}

function parseDocString (docs) {
  var lines = docs.split('\n')
  var grouped = []
  var section = ''
  var buff = ''
  var i = 0
  var line
  while (i < lines.length) {
    line = lines[i]
    i++
    if (/^#/.test(line)) {
      grouped.push({
        section: section,
        text: buff
      })
      section = line + '\n'
      buff = ''
    } else if (line.trim() === 'cb:') {
      grouped.push({
        section: section,
        text: buff
      })
      section = 'returns'
      buff = ''
      while (i < lines.length) {
        line = lines[i]
        if (!/^- /.test(line)) {
          break
        }
        i++
        if (/^- xcommand_handle:/.test(line)) {
          // ignore line
        } else if (/^- err:/.test(line)) {
          // ignore line
        } else {
          buff += line + '\n'
        }
      }
    } else {
      buff += line + '\n'
    }
  }
  grouped.push({
    section: section,
    text: buff
  })
  var keyed = {
    desc: '',
    params: '',
    returns: '',
    errors: ''
  }
  grouped.forEach(function (o) {
    var section = o.section
      .toLowerCase()
      .replace(/[^a-z0-9_]+/g, ' ')
      .replace(/\s+/, ' ')
      .trim()
    if (section === '') {
      section = 'desc'
    }
    if (section === 'return') {
      section = 'returns'
    }
    if (!keyed.hasOwnProperty(section)) {
      throw new Error('Unsupported doc string section: ' + section)
    }
    keyed[section] += o.text + '\n'
  })
  var ast = {
    desc: keyed.desc.trim(),
    params: parseDocStringParams(keyed.params),
    returns: parseDocStringReturns(keyed.returns),
    errors: parseDocStringErrors(keyed.errors)
  }
  return ast
}

function parseDocStringParams (params) {
  var lines = params.split('\n')
  var grouped = []
  var curr = {
    name: '',
    optional: false,
    text: ''
  }

  // json examples that share a line
  lines = lines.map(function (line) {
    return line.replace(/([^"])\s*:\s*{\s*$/, '$1:\n{')
  }).join('\n').split('\n')

  lines.forEach(function (line) {
    if (line.trim().length === 0) {
      return
    }
    if (line.trim() === 'cb: Callback that takes command result as parameter.') {
      return
    }

    // sublists
    line = line
      .replace(/^- /, '  * ')
      .replace(/^ {2}- /, '    * ')

    var m = /^([a-zA-Z0-9_]+)\s*(\([^)]*\)\s*)?:(.*)$/.exec(line)
    if (m) {
      if (m[2] && !/optional/i.test(m[2])) {
        throw new Error('Expected param (optional): ' + line)
      }
      grouped.push(curr)
      curr = {
        name: m[1],
        optional: !!m[2],
        text: m[3] + '\n'
      }
    } else {
      curr.text += line + '\n'
    }
  })
  grouped.push(curr)

  var ast = {}
  grouped.forEach(function (o) {
    if (o.name.trim().length === 0 && o.text.trim().length === 0) {
      return
    }
    if (!ast[o.name]) {
      ast[o.name] = {text: '', optional: false}
    }
    ast[o.name].text += o.text + '\n'
    ast[o.name].optional = ast[o.name].optional || o.optional
  })
  Object.keys(ast).forEach(function (name) {
    ast[name].text = ast[name].text.replace(/\s*$/, '')
  })
  return ast
}

function parseDocStringReturns (src) {
  var out = src
    .replace(/^\s*Error\s*Code/, '')
    .trim()
  switch (src.toLowerCase().replace(/[^a-z]+/g, '')) {
    case 'none':
    case 'errorcode':
    case 'requestresultasjson':
      // remove comments that say nothing novel
      out = ''
      break
  }
  out = out
    .replace(/^-\s*[a-z0-9_]+\s*[-:]/, '')
    .trim()
  return out
}

function parseDocStringErrors (src) {
  var lines = src.split('\n')
  return lines
    .map(line => line.trim())
    .filter(line => line.length > 0)
    .map(function (line) {
      if (!/^[a-z]+\*$/i.test(line)) {
        throw new Error('Invalid error doc string: ' + line)
      }
      return line
    })
}

var readmeOut = []
var inBlock = false
fs.readFileSync(README_FILE, 'utf8').split('\n').forEach(function (line) {
  if (/CODEGEN-START/.test(line)) {
    readmeOut.push(line)
    readmeOut.push(readme)
    inBlock = true
  }
  if (/CODEGEN-END/.test(line)) {
    inBlock = false
  }
  if (!inBlock) {
    readmeOut.push(line)
  }
})

fs.writeFileSync(README_FILE, readmeOut.join('\n'), 'utf8')
