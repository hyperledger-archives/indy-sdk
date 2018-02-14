import * as ffi from 'ffi'
import * as path from 'path'

import { FFIConfiguration, IFFIEntryPoint } from './rustlib'

export interface ICXSRuntimeConfig {
  basepath?: string
}

// CXSRuntime is the object that interfaces with the cxs sdk functions
// FFIConfiguration will contain all the sdk api functions
// CXSRuntimeConfg is a class that currently only contains a chosen basepath for the .so file
// I made it a class just in case we think of more needed configs

export class CXSRuntime {
  readonly ffi: IFFIEntryPoint
  private _config: ICXSRuntimeConfig

  constructor (config: ICXSRuntimeConfig = {}) {
    this._config = config
     // initialize FFI
    const libraryPath = this._initializeBasepath()
    this.ffi = ffi.Library(libraryPath, FFIConfiguration)
  }

  private _initializeBasepath = (): string => this._config.basepath || path.resolve(__dirname, '/usr/lib/libcxs.so')
}
