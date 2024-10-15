# TMFLib

![Cargo Build](https://github.com/rruckley/tmflib/actions/workflows/rust.yml/badge.svg)

## TMF Library for Rust

### Description

This library covers data structures required to interact with various TMForum APIs.
It does not define any persistence nor provide a REST interface (at this stage)
but simply provides definitions of all the schema and helpful functions and traits to create and maniuplate compliant objects
that can then be seriliased into or from JSON as required.

### API Version Features

By default this crate will compile v4 versions of APIs.

* **build-V4**

This is the default version compiled

* **build-V5**

This flag can be enabled to compile v5 APIs where available, mutually exclusive with build-V4.

### Common Feature

Within the library is a set of common modules. These modules refer to other TMF modules and thus all
modules referenced by the common module are included under this feature.
Specifically:
    - tmf620
    - tmf629
    - tmf632
    - tmf666
    - tmf667
    - tmf669
    - tmf674

### ODA Component Features

All ODA Component identifies, e.g. TMFC001 have been mapped onto features to enable building the library
to support a specific component.

*NB: For components that dont' have any defined APIs, a common set of APIs are included under the **common** feature*
