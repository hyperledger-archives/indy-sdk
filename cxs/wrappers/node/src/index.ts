import { FFIEntryPoint,FFIConfiguration, FFIInterfaceConfig } from './rustlib'
import * as ffi from 'ffi'

export interface CXSAcessType {
    readonly ffi: FFIEntryPoint
}

class CXSRuntimeConfig {
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
                basepath = "/node_modules/cxs/lib/libcxs.so"
            }

            return basepath
        }

        // initialize FFI
        const libraryPath = _initialize_basepath()
        this.ffi = ffi.Library(libraryPath, FFIConfiguration)
    }
}
// var basepath = "/node_modules/cxs/lib/libcxs.so"
// var basepath = "/home/rmarsh/dev/cxs/cxs/wrappers/node/lib/libcxs.so";

var run = new CXSRuntime(new CXSRuntimeConfig())
// var run = new CXSRuntime(new CXSRuntimeConfig(basepath))
run.ffi.cxs_init();

