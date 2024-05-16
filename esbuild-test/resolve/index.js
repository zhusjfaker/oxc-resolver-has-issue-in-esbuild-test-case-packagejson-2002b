const resolve = require('enhanced-resolve');
const path = require('path');

const resolver = resolve.create({
  cache: false,  // 禁用缓存
  aliasFields: ['browser'],
  // exportsFields: ['exports'],
});
// const resolvePath = path.resolve(__dirname, '../src');
// const spec = 'pkg/sub';
// const resolvePath = path.resolve(__dirname, "../src/node_modules/pkg/sub");
// const spec = 'sub'

const resolvePath =
  '/Users/jason.zhu/Desktop/github/rolldown/crates/rolldown/tests/esbuild/packagejson/test_package_json_exports_default_over_import_and_require/src';
const spec = 'pkg';

resolver(resolvePath, spec, (err, result) => {
  console.log('resolve \n', result, '\n', err);
});
