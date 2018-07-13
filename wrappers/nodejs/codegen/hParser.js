var fs = require('fs')
var path = require('path')
var stringify = require('json-stringify-pretty-compact')

var api = {
  errors: {},
  functions: {}
}

var dir = path.resolve(__dirname, '../../../libindy/include')
fs.readdirSync(dir).forEach(function (file) {
  file = path.resolve(dir, file)

  var group = path.basename(file).replace(/^indy_|\.h$/g, '')
  var hText = fs.readFileSync(file, 'utf8') + '\n'
  parseSection(group, hText)
})

function parseSection (group, hText) {
  // split it into lines and remove comments and # lines
  var lines = hText
    .split('\n')
    .map(function (line) {
      return line
        .replace(/\s+/g, ' ')
        .trim()
        .replace(/^#.*$/g, '')
        .replace(/\s+/g, ' ')
        .trim()
    })
    .map(function (line) {
      return /^\/\//.test(line)
        ? line
        : line.replace(/\/\/.*$/g, '').trim()
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

  var externCLines = externCText.split('\n')
  var docs = ''
  var externFn = ''
  var functions = []
  i = 0
  while (i < externCLines.length) {
    line = externCLines[i]
    i++
    while (/^\/\//.test(line)) {
      docs += line.replace(/^\/\/+/, '').trim() + '\n'
      line = externCLines[i]
      i++
    }
    if (/^extern/.test(line)) {
      while (i < externCLines.length) {
        if (/^\/\//.test(line)) {
          break
        }
        if (/;$/.test(line)) {
          externFn += line + ' '
          break
        }
        externFn += line + ' '
        line = externCLines[i]
        i++
      }
      functions.push({
        docs: docs.trim(),
        externFn: externFn.replace(/\s+/, ' ').trim()
      })
      docs = ''
      externFn = ''
    }
  }
  functions.forEach(function (fn) {
    var m = /^extern (indy_error_t) (\w+) ?\((.*\));$/.exec(fn.externFn)
    if (!m) {
      throw new Error('Unexpected function line: ' + fn.externFn)
    }
    api.functions[m[2]] = {
      docs: fn.docs,
      group: group,
      params: parseParams(m[3]),
      ret: m[1]
    }
  })
}

// parse a function params string until it hits the closing ")"
function parseParams (src) {
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
    switch (o.type.toLowerCase().replace(/\s+/g, '').trim()) {
      case 'indy_u64_t':
      case 'longlong':
      case 'unsignedlonglong':
        if (o.name === 'timestamp') {
          o.timestamp = true
        }
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

// parse docs from rust code api
dir = path.resolve(__dirname, '../../../libindy/src/api')
fs.readdirSync(dir).forEach(function (file) {
  file = path.resolve(dir, file)

  var rustSrc = fs.readFileSync(file, 'utf8')

  var lines = rustSrc.split('\n')
  var i = 0
  var docs = ''
  while (i < lines.length) {
    let line = lines[i]
    i++
    while (/^\/\//.test(line) && i < lines.length) {
      docs += line.replace(/^\/\/+ ?/, '').replace(/\s+$/, '') + '\n'
      line = lines[i]
      i++
    }
    var m = /^pub *extern *fn *([a-zA-Z0-9_]+) *\(/.exec(line)
    if (m) {
      let fnName = m[1].trim()
      if (api.functions[fnName]) {
        api.functions[fnName].docs = docs.trim()
      }
      docs = ''
    }
  }
})

// manually set some json and timestamp hints
api.functions.indy_build_attrib_request.params[4].json = true
api.functions.indy_build_cred_def_request.params[2].json = true
api.functions.indy_build_node_request.params[3].json = true
api.functions.indy_build_revoc_reg_def_request.params[2].json = true
api.functions.indy_build_revoc_reg_entry_request.params[4].json = true
api.functions.indy_build_schema_request.params[2].json = true
api.functions.indy_create_pool_ledger_config.params[2].json = true
api.functions.indy_create_wallet.params[4].json = true
api.functions.indy_create_wallet.params[5].json = true
api.functions.indy_delete_wallet.params[2].json = true
api.functions.indy_get_my_did_with_meta.params[3].params[2].json = true
api.functions.indy_import_wallet.params[4].json = true
api.functions.indy_import_wallet.params[5].json = true
api.functions.indy_issuer_create_credential.params[6].optional = true
api.functions.indy_issuer_create_schema.params[4].json = true
api.functions.indy_issuer_merge_revocation_registry_deltas.params[3].params[2].json = true
api.functions.indy_list_my_dids_with_meta.params[2].params[2].json = true
api.functions.indy_list_pairwise.params[2].params[2].json = true
api.functions.indy_list_pools.params[1].params[2].json = true
api.functions.indy_list_wallets.params[1].params[2].json = true
api.functions.indy_open_wallet.params[3].json = true
api.functions.indy_parse_get_cred_def_response.params[1].json = true
api.functions.indy_parse_get_revoc_reg_def_response.params[1].json = true
api.functions.indy_parse_get_revoc_reg_delta_response.params[1].json = true
api.functions.indy_parse_get_revoc_reg_response.params[1].json = true
api.functions.indy_parse_get_schema_response.params[1].json = true

api.functions.indy_build_get_revoc_reg_delta_request.params[3].timestamp = true
api.functions.indy_build_get_revoc_reg_delta_request.params[4].timestamp = true

fs.writeFileSync(path.resolve(__dirname, 'api.json'), stringify(api, {maxLength: 100}), 'utf8')
