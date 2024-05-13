
# Issue Descriptions

## retrace one's steps

```sh

cd ./esbuild-test && yarn && npm test 

// you can confirm esbuild is normal and safety, it's not happend any error in build process

cd ../ && cargo test 

// it happend rust panic in case "./test/mod.rs"  case -> "test_resolver_inner_package_json"

```

## Sources of test code

https://github.com/evanw/esbuild/blob/main/internal/bundler_tests/bundler_packagejson_test.go#L830