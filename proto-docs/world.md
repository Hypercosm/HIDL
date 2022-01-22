# Extension `world`
*v0.1.0*


## Interfaces
### Interface `world`

#### Events
##### `add_entities(entities: []EntityInfo)`

##### `update_entities(entities: []EntityInfo)`

##### `remove_entities(entities: []Entity)`

### Interface `Entity`

#### Methods
##### `interact()`

## Types
### Struct `EntityInfo`

- `asset_id`: `uuid`
- `entity`: `Entity`
- `transformation`: `matrix4x4`
- `attrs`: `EntityAttrs`
### Flags `EntityAttrs`

- `None = 0`
- `Interactable = 1`
- `Collidable = 10`
