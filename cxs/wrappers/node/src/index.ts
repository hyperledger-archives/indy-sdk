import * as ffi from 'ffi'
import * as path from 'path'
import { CXSRuntimeConfig, FFIConfiguration, FFIEntryPoint } from './rustlib'

export interface ICXSAcessType {
  readonly _ffi: FFIEntryPoint
}

// CXSRuntime is the object that interfaces with the cxs sdk functions
// FFIConfiguration will contain all the sdk api functions
// CXSRuntimeConfg is a class that currently only contains a chosen basepath for the .so file
// I made it a class just in case we think of more needed configs

export class CXSRuntime implements ICXSAcessType {
  readonly _ffi: FFIEntryPoint
  private _config: CXSRuntimeConfig

  constructor (config: CXSRuntimeConfig = {}) {
    this._config = config
     // initialize FFI
    const libraryPath = this._initializeBasepath()
    this._ffi = ffi.Library(libraryPath, FFIConfiguration)
  }

  private _initializeBasepath = (): string => this._config.basepath || path.resolve(__dirname, '../lib/libcxs.so')
}
export { init_cxs } from './api/init'
export { Connection } from './api/connection'
export { CXSRuntimeConfig } from './rustlib'
export * from './api/api'
