# Extension asset_delivery
*v0.1.0*

Delivers assets

## Interfaces
### asset_delivery
Singleton for asset delivery

#### Events
##### `load_assets(assets: []Asset)`

##### `unload_assets(assets: []uuid)`

#### Methods
##### `fetch_by_id(id: uuid) -> Asset`

##### `fetch_by_name(name: string) -> Asset`

##### `fetch_by_ids(ids: []uuid) -> []Asset`

##### `fetch_by_names(names: []string) -> []Asset`

##### `get_id(name: string) -> uuid`

## Types
### struct `Asset`
 An asset: the type is given by TODO 
- `id`: `uuid`
- `name`: `string`
- `data`: `bytes`
