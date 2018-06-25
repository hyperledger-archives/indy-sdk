import * as path from 'path'

// @ts-ignore
import * as appModulePath from 'app-module-path'
appModulePath.addPath(path.resolve(__dirname, '../'))
appModulePath.addPath(__dirname)
