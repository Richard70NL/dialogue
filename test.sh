#!/bin/sh

psql -c 'drop schema if exists dialogue cascade;' -d dialogue_test -U dialogue_test

cargo run -- install --verbose --test-data --database-url=postgresql://dialogue_test@localhost/dialogue_test

cargo test
