# Entities
These are `rust` types generated from the db schema using the `sea-orm-cli`. Make sure your database is running, and has the correct schema you would like to generate `entities` for.

## Generating Entities
To generate entities you will need the `sea-orm-cli` setup on your local machine. Once that is done you will want to generate entities.

Below is an example of how to do that. 

__NOTE__: This will replace existing files in the `entity/src` directory. So if an application specific change was made to an entity. Make sure it's re-exported from, and custom logic is implemented in a different location.

```shell
# generate an entity for one table with `serde` enabled for serialization/deserialization.
$ sea-orm-cli generate entity \
    --with-serde=both \
    --output-dir entity/src \
    --tables TABLE_NAMES

# WARNING - Generate all new entities from the whole database schema. Make sure you know what your doing.
$ sea-orm-cli generate entity \
    --with-serde=both \
    --output-dir entity/src
```