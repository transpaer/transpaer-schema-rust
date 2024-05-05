#!/bin/sh

cargo typify schema/sustainity.json -a PartialEq -a Eq -o sustainity-schema/src/models.rs
