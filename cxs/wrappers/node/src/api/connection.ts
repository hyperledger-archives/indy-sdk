import * as index from '../index'
import * as ref from 'ref'
import * as Struct from 'ref-struct'
import {
    Connections
} from './api'
import {
    Errorcode,
}from './mod'

export class Connection implements Connections{

    readonly RustAPI : any
    connection_handle : any

    constructor(path?: string){
        this.RustAPI = new index.CXSRuntime(new index.CXSRuntimeConfig(path)).ffi
    }

    connection_create(recipient_info: string, connection_handle: number) : number{
        let connection_handle_ptr = connection_handle
        if(typeof connection_handle == "number"){
            // console.log(connection_handle)
            connection_handle_ptr = ref.alloc(ref.types.int, connection_handle)
            var old_handle = ref.alloc(ref.types.int, connection_handle)
            // console.log(connection_handle_ptr)
        }

        var result = this.RustAPI.cxs_connection_create(recipient_info, connection_handle_ptr)
        // console.log(old_handle)
        // console.log(connection_handle_ptr)
        // console.log(connection_handle_ptr == old_handle)
        this.connection_handle = ref.deref(connection_handle_ptr)
        return result
    }

    connection_connect(connection_handle: number): Errorcode{
        return this.RustAPI.cxs_connection_connect(connection_handle)
    }

    connection_get_data(connection_handle: number): string{

       return this.RustAPI.cxs_connection_get_data(connection_handle)
    }
}
//
//
// export function connection_create(): Errorcode{
//
//     return Errorcode.Failure
// }
//
// export function connection_connect(): Errorcode{
//
//     return Errorcode.Failure
// }
//
// export function connection_get_data(): Errorcode{
//
//     return Errorcode.Failure
// }
//
// export function connection_get_state(): Errorcode{
//
//     return Errorcode.Failure
// }
//
// export function connection_list_state(): Errorcode{
//
//     return Errorcode.Failure
// }