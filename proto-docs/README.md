# Hypercosm Protocol Docs
## Interfaces
### Interface `Object`
*v1.0.0*

The root interface, that all interfaces inherit from and
all objects implement

#### Methods
##### `list_interfaces() -> []string`
Get all interfaces implemented by an object

The interfaces will be listed in the format `{namespace}.{extension}.{interface_name}.{version}`

##### `release()`
Remove the object from the object list

Future attempts to make calls on the object ID **MUST** fail.

The resources associated with the object may be released, but the
object ID must not be reused, we have plenty of them

It is an error to remove the root singleton (id 0)

### Interface `Root`
*v0.1.0*

Singleton with known id 0

This is the only object that is known at the start of connection
and is used to discover other objects

#### Methods
##### `list_extensions() -> []string`
List the extensions implemented by the conected node

##### `ping()`
Check the conection status

##### `get_object_by_id(id: uuid) -> object`

##### `get_object_by_name(name: string) -> object`
Get the id of a singleton by the name of the interface

## Extensions
- [asset_delivery](asset_delivery.md)
- [world](world.md)
- [execution_context](execution_context.md)
