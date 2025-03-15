# rsbuf

----

## Contributor Notes

```shell
wasm-pack build --target nodejs --out-dir dist --out-name rsbuf # Builds the wasm bundle.
~/IdeaProjects/wabt/bin/wasm2wat  dist/rsbuf_bg.wasm -o src/rsbuf.wat # Generates the wat file.
npm publish --access public # Publishes the version of this to npm.
```