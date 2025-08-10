#!/bin/sh

cargo typify schema/transpaer.json -a PartialEq -a Eq -o transpaer-schema/src/models.rs
