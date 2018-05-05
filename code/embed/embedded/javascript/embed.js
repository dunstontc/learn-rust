const ffi = require('ffi-napi');

const src = '../../target/release/libembed';

const lib = ffi.Library(src, {
  'process': ['void', []],
});

lib.process();

console.log('done!');

