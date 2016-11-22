#!/bin/sh

set -e

(cat release-react-native/wrapper.prefix && cat target/release-react-native/datomish.bare.js && cat release-react-native/wrapper.suffix) > target/release-react-native/datomish.js

echo "Packed target/release-react-native/datomish.js"
