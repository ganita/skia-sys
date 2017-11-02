#!/bin/sh

BASEDIR=$(dirname "$0")

bindgen $BASEDIR/sk_includes.h -o $BASEDIR/src/bindings.rs
