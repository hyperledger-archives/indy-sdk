/**
 * Node FFI wrapper around .so postgres storage plugin
 */
import * as ffi from 'ffi';
import * as ref from 'ref';
const int = ref.types.int;

const postgresPlugin = ffi.Library(
    './resources/libindystrgpostgres.so',
    {
        init: [int, []],
        setStoragetype: [int, ['string', 'string']],
    },
);
export = postgresPlugin;
