# Extension asset_delivery
Delivers assets

## Interfaces
### asset_delivery
*v0.1.0*

Singleton for asset delivery

#### Methods
##### `fetch_by_id(id: uuid,)`

##### `fetch_by_name(name: string,)`

##### `fetch_by_ids(ids: []uuid,)`

##### `fetch_by_names(names: []string,)`

##### `getId(name: string,)`

#### Events
##### `load_assets(assets: []Asset,)`

##### `unload_assets(assets: []uuid,)`

