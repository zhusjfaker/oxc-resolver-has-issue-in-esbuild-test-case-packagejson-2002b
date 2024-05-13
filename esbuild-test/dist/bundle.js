(() => {
  var __getOwnPropNames = Object.getOwnPropertyNames;
  var __commonJS = (cb, mod) => function __require() {
    return mod || (0, cb[__getOwnPropNames(cb)[0]])((mod = { exports: {} }).exports, mod), mod.exports;
  };

  // src/node_modules/pkg/sub/bar.js
  var require_bar = __commonJS({
    "src/node_modules/pkg/sub/bar.js"() {
      function works() {
      }
      works();
    }
  });

  // src/node_modules/pkg/sub/foo.js
  var require_foo = __commonJS({
    "src/node_modules/pkg/sub/foo.js"() {
      require_bar();
    }
  });

  // src/entry.js
  require_foo();
})();
//# sourceMappingURL=bundle.js.map
