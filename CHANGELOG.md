# 0.2

* expose security definition as an enum type
* expose schema types which define `parameters` with new `ParameterOrRef` type
* Adds License object
* Adds Contact object
* Derives Default for all structs
* Derives Clone for all structs
* Changes the order of the output to be more similar to OpenAPI examples
* switch to 2018 edition
* swap error_chain for failure crate

# 0.1.5

* expose other schema types as public interfaces
* re-export Result and ResultExt as top level interfaces

# 0.1.4

* added operational `parameters` field to `Operations` object

# 0.1.3

* added optional `required` and `enum_values` fields to `Schema` object

# 0.1.2

* added optional `format` fields to `Parameter` object

# 0.1.1

* added optional `summary` field to `Operation` object
* made schemes and tags optional fields on `Operation` object

# 0.1.0

* initial release
