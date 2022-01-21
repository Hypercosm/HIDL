# Extension execution_context
*v0.1.0*


## Interfaces
### execution_context
*v0.1.0*


#### Methods
##### `load_wasm_module(assetId: uuid, exports: [string]string) -> vu64`

##### `load_lua_script(assetId: uuid) -> vu64`

##### `load_inline_lua_script(script: string) -> vu64`

##### `begin_execution(module_or_script: vu64, entry_point: string)`

##### `begin_inline_lua_execution(script: string)`

