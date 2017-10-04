import { FFIEntryPoint, FFIConfiguration } from './rustlib'
import * as ffi from 'ffi'
import * as path from 'path'

export interface CXSAcessType {
    readonly ffi: FFIEntryPoint
}

export class CXSRuntimeConfig {
    basepath?:string
    constructor(_basepath?: string){
        this.basepath = _basepath
    }

}

/** CXSRuntime is the object that interfaces with the cxs sdk functions **/
/** FFIConfiguration will contain all the sdk api functions **/
/** CXSRuntimeConfg is a class that currently only contains a chosen basepath for the .so file
 * I made it a class just in case we think of more needed configs **/

export class CXSRuntime implements CXSAcessType {
    readonly ffi: FFIEntryPoint

    constructor(config?: CXSRuntimeConfig) {
        config = config || {}

        function _initialize_basepath(): string {
            let basepath = config.basepath

            if (basepath === undefined) {
                //This basepath is in the local/appSpecific node_modules
                basepath = path.resolve("../node_modules/cxs/lib/libcxs.so")
            }

            return basepath
        }

        // initialize FFI
        const libraryPath = _initialize_basepath()
        this.ffi = ffi.Library(libraryPath, FFIConfiguration)
    }
}
