# TMFLib

## TMF Library for Rust
### Description
This library covers data structures required to interact with various TMForum APIs.
It does not define any persistence nor provide a REST interface (at this stage)
but simply provides definitions of all the schema and helpful functions and traits to create and maniuplate compliant objects
that can then be seriliased into or from JSON as required.
 
### API Version Features 
By default this crate will compile v4 versions of APIs. 
* **v4**
This is the default version compiled
* **v5**
This flag can optionally be enabled to compile v5 APIs where available
