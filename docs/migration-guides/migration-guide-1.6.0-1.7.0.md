<!-- markdownlint-disable MD033 -->

# Libindy 1.6 to 1.7 migration Guide

This document is written for developers using Libindy to provide necessary information and
to simplify their transition to Libindy 1.7 from Libindy 1.6. If you are using older Libindy
version you can check migration guides history:

* [Libindy 1.3 to 1.4 migration](https://github.com/hyperledger/indy-sdk/blob/v1.4.0/doc/migration-guide.md)
* [Libindy 1.4 to 1.5 migration](https://github.com/hyperledger/indy-sdk/blob/v1.5.0/doc/migration-guide-1.4.0-1.5.0.md)
* [Libindy 1.5 to 1.6 migration](https://github.com/hyperledger/indy-sdk/blob/v1.6.0/doc/migration-guide-1.5.0-1.6.0.md)
## Table of contents

* [Notes](#notes)
* [Libindy 1.6 to 1.7.0 migration](#libindy-16-to-170-migration-guide)
    * [Logger API](#logger-api)
    * [Libindy API](#libindy-api)

## Notes

Migration information is organized in tables, there are mappings for each Libindy API part of how older version functionality maps to a newer one.
Functions from older version are listed in the left column, and the equivalent newer version function is placed in the right column:

* If some function had been added, the word 'NEW' would be placed in the left column.
* If some function had been deleted, the word 'DELETED' would be placed in the right column.
* If some function had been deprecated, the word 'DEPRECATED' would be placed in the right column.
* If some function had been changed, the current format would be placed in the right column.
* If some function had not been changed, the symbol '=' would be placed in the right column.
* To get more details about current format of a function click on the description above it.
* Bellow are signatures of functions in Libindy C API.
  The params of ```cb``` (except command_handle and err) will be result values of the similar function in any Libindy wrapper.

## Libindy 1.6 to 1.7.0 migration Guide

### Logger API

The main purpose of this API is to forward logs of libindy and wrappers to its consumers. It is needed if you consume libindy as a `.so` or `.dll` - so you can forward logs from libindy to your logging framework.
You don't need this endpoints if you use libindy through the wrapper -- in Java, Rust and Python wrappers they are already forwarded to `slf4j` for Java, `log` crate for Rust and default logging facade for python.     

<table>
    <tr>  
      <th>v1.6.0 - Logger API</th>
      <th>v1.7.0 - Logger API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/libindy/src/api/logger.rs#L26">
              Set custom logger implementation
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
              indy_set_logger(context: *const c_void,
                              enabled: Option<fn(context: *const c_void,
                                                 level: u32,
                                                 target: *const c_char) -> bool>,
                              log: Option<fn(context: *const c_void,
                                             level: u32,
                                             target: *const c_char,
                                             message: *const c_char,
                                             module_path: *const c_char,
                                             file: *const c_char,
                                             line: u32)>,
                              flush: Option<fn(context: *const c_void)>) -> ErrorCode
          </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/libindy/src/api/logger.rs#L56">
              Set default logger implementation.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
              indy_set_default_logger(pattern: *const c_char) -> ErrorCode
          </pre>
      </td>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/libindy/src/api/logger.rs#L85">
              Get the currently used logger.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>
              indy_get_logger(context_p: *mut *const c_void,
                              enabled_cb_p: *mut Option<fn(context: *const c_void,
                                                           level: u32,
                                                           target: *const c_char) -> bool>,
                              log_cb_p: *mut Option<fn(context: *const c_void,
                                                       level: u32,
                                                       target: *const c_char,
                                                       message: *const c_char,
                                                       module_path: *const c_char,
                                                       file: *const c_char,
                                                       line: u32)>,
                              flush_cb_p: *mut Option<fn(context: *const c_void)>)
          </pre>
      </td>
    </tr>
</table>

### Libindy API

The main purpose of this API is to set Liibndy configuration.
<table>
    <tr>  
      <th>v1.6.0 - Libindy API</th>
      <th>v1.7.0 - Libindy API</th>
    </tr>
    <tr>
      <th colspan="2">
          <a href="https://github.com/hyperledger/indy-sdk/blob/v1.7.0/libindy/src/api/mod.rs#L243">
              Set libindy runtime configuration.
          </a>
      </th>
    <tr>
    <tr>
      <td>
          <b>NEW</b>
      </td>
      <td>
          <pre>indy_set_runtime_config(config: *const c_char) -> ErrorCode</pre>
      </td>
    </tr>
</table>