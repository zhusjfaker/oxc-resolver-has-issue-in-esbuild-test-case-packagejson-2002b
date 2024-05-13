const resolve = require('enhanced-resolve');
const path = require('path');

const resolver = resolve.create({
  aliasFields: ['browser'],
});
// const resolvePath = path.resolve(__dirname, '../src');
// const spec = 'pkg/sub';
const resolvePath = path.resolve(__dirname, "../src/node_modules/pkg/sub");
const spec = 'sub'

resolver(resolvePath, spec, (err, result) => {
  console.log('resolve success \n', result, '\n', err);
});
