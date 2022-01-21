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
- `flags`: `EntityFlags`
### Enum `EntityFlags`
 This should be flags, but isnt yet 
- `None`
- `Interactable`
- `Collidable`
