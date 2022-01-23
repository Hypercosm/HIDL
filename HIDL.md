# Hypercosm Interface Description Language (HIDL)

⚠️ **extremely serious warning:** this is _pre-alpha_, proof-of-concept
software! everything here has _no stability guarantees_ &mdash; the HIDL
language, the contents of the HIDL files, and the JSON representation in this
repository are not guaranteed to be interoperable except within the same Git
revision.

This document describes HIDL. Right specificly, it describes the non-obvious
parts of HIDL. Note that what counts as obvious is entirely subjective, and
depends heavily on your past experiences and how you think about software. If
theirs something about HIDL, please ask, and it will be added here. Initialy
this is just for my notes, but the goal is to expand it to fully descirbe HIDL,
but for now it doesnt.

## Networking and definitions

Hypercosm networks consist of Cosms and Browsers. 

- **Cosms** are servers and host a world to the browser. They (may) connect to
  several browsers at once.
- **Browsers** are clients and get the world from a server. They initiate the
  connection and connect to one Cosm at a time.
- A **Node** is either a Browser or a Cosm.
- A **World** is a virtual space shared between one Cosm and one or more
  Browsers.
- **Hypercosm** is the protocol that defines how Browsers and Cosms communicate
  to create a World
- Hypercosm Interface Discription Language (**HIDL**) describes the call's and
  responces that Browsers and servers can make to eachother. HIDL files define
  some, but not all of the hypercosm protocol

## Object model

Their are 4 parts of the object model

- Extensions
- Objects
- Handles
- Callsets

### Extensions

All HIDL code is organised in extensions, which all live in their 
own file. Core (`hypercosm.core`) is tecnicly modeled
as an extension, even though it is mandatory. 

Each extension exists in a namespace, which may be nested a la java packages.

The offical hypercosm extensions exists in the `hypercosm` namespaces, eg `hypercosm.core`,
`hypercosm.asset_delivery`

Each extesnions consists of a callset, and potentialy some
objects and types. 

Each extension has a handle, which can be found with
`hypercosm.core.get_extension_handles()` The `hypercosm.core` extension always
has handle 0.

### Objects

An object is an opaque reference to some state on the other node.
Each object has an ID, which is used as the handle. This means an object
ID must be unique, and not overlap with the hanldes for extensions.

Objects have a callset, which is the methods that may be called by them.

To get an object handle, call a method that returns one. These will usualy
be methods on the extensions.

OO programmers may model the callset of an extension as a singleton object.

TODO: How should we differenciate the name of an object type vs an object
instance. Should we do class/object, or something else?

Objects cannot inherit. The methods that existed on the old object
instance will now exist on the `hypercosm.core` callset

### Callsets

A callset consists of methods and events. Both extensions and objects have a
callset. 

### Handles

A handle is a 64 bit unsigned integer, ecoded as a `vu64`. A handle either represents
an object ID, or the callset of an extension.

To make a call, the caller sends the handle they want to make the call on,
and the method id for the call they want to make.


## Types

Objects can only be passed by ID, because they are opaque,

Types are data that can be passed, but cant have methods.

Types specify the data layout

Types can be struct (rust struct), enum (rust enum, but with no data in field), or flags (rust bitflags)


