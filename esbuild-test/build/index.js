const esbuild = require('esbuild');
const path = require('path');

esbuild
  .build({
    entryPoints: [path.resolve(__dirname, '../src/entry.js')],
    bundle: true,
    minify: false,
    sourcemap: true,
    target: [],
    write: true,
    outfile: path.resolve(__dirname, '../dist/bundle.js')
  })
  .then((res) => {
    console.log('build success \n', res);
  })
  .catch((err) => {
    console.log('build error \n', err);
  });
