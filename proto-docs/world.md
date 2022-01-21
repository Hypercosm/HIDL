# Extension world
*v0.1.0*


## Interfaces
### world
*v0.1.0*


#### Events
##### `add_entities(entities: []EntityInfo)`

##### `update_entities(entities: []EntityInfo)`

##### `remove_entities(entities: []Entity)`

### Entity
*v0.1.0*


#### Methods
##### `interact()`

## Types
### struct `EntityInfo`

- `assetId`: `uuid`
- `entity`: `Entity`
- `transformation`: `matrix4x4`
- `flags`: `EntityFlags`
### enum `EntityFlags`
 This should be flags, but isnt yet 
- `None`
- `Interactable`
- `Collidable`
