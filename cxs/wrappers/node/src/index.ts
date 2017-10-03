import { FFIEntryPoint,FFIConfiguration, FFIInterfaceConfig } from './rustlib'
import * as ffi from 'ffi'

export interface CXSAcessType {
    readonly ffi: FFIEntryPoint
}

export class CXSRuntimeConfig {
    basepath?:string
    constructor(_basepath?: string){
        this.basepath = _basepath
    }

}

export class CXSRuntime implements CXSAcessType {
    readonly basepath: string
    readonly ffi: FFIEntryPoint

    constructor(config?: CXSRuntimeConfig) {
        config = config || {}

        function _initialize_basepath(): string {
            // this needs additional logic
            let basepath = config.basepath

            if (basepath === undefined) {
                basepath = "/usr/local/lib/node_modules/cxs/lib/libcxs.so"
            }

            return basepath
        }

        // initialize FFI
        const libraryPath = _initialize_basepath()
        this.ffi = ffi.Library(libraryPath, FFIConfiguration)
    }
}


