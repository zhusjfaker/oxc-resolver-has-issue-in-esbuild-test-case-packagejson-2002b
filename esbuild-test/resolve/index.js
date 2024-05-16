const resolve = require('enhanced-resolve');
const path = require('path');

const resolver = resolve.create({
  cache: false,  // 禁用缓存
  // aliasFields: ['browser'],
  alias:{
    "#fs": "fs",
    "#http": "node:http"
  }
  // conditionNames: ['node', 'require'],
  // exportsFields: ['exports'],
});
// const resolvePath = path.resolve(__dirname, '../src');
// const spec = 'pkg/sub';
// const resolvePath = path.resolve(__dirname, "../src/node_modules/pkg/sub");
// const spec = 'sub'

const resolvePath =
  '/Users/jason.zhu/Desktop/github/rolldown/crates/rolldown/tests/esbuild/packagejson/test_package_json_subpath_import_node_builtin_issue3485';
const spec = '#fs';


resolver(resolvePath, spec, (err, result) => {
  console.log('resolve \n', result, '\n', err);
});
