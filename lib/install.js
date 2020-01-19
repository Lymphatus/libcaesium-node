const os = require('os');
const log = require('logalot');
const {execSync} = require('child_process');
const Git = require('nodegit');
const binBuild = require('bin-build');

const fs = require('fs');

const cpuNum = os.cpus().length;

if (!fs.existsSync(__dirname + '/../build/mozjpeg/lib/libjpeg.a')) {
  binBuild.url('https://github.com/mozilla/mozjpeg/archive/426de82d0c081c996c23b75fed05833b6627b590.tar.gz', [
    'cmake -DCMAKE_INSTALL_PREFIX="' + __dirname + '/../build/mozjpeg" .',
    'make -j' + cpuNum,
    'make install -j' + cpuNum,
  ]).then(() => {
    log.success('mozjpeg built successfully');
  }).catch(error => {
    log.error(error.stack);
  });
} else {

  log.success('mozjpeg found');
}

if (!fs.existsSync(__dirname + '/../build/zopflipng/lib/libzopflipng.a')) {
  binBuild.url('https://github.com/google/zopfli/archive/zopfli-1.0.3.tar.gz', [
    'cmake -DCMAKE_INSTALL_LIBDIR="' + __dirname + '/../build/zopflipng/lib" .',
    'make -j' + cpuNum,
    'make install -j' + cpuNum,
  ]).then(() => {
    log.success('zopflipng built successfully');
  }).catch(error => {
    log.error(error.stack);
  });
} else {
  log.success('zopflipng found');
}

if (!fs.existsSync(__dirname + '/../build/libcaesium/lib/libcaesium.a')) {
// binBuild.url('https://github.com/Lymphatus/libcaesium/archive/v0.5.0.tar.gz', [
  binBuild.url('https://github.com/Lymphatus/libcaesium/archive/static.tar.gz', [
    'cmake -DCMAKE_INSTALL_PREFIX="' + __dirname + '/../build/libcaesium/" .',
    'make -j' + cpuNum,
    'make install -j' + cpuNum,
  ]).then(() => {
    log.success('libcaesium built successfully');
  }).catch(error => {
    log.error(error.stack);
  });
} else {

  log.success('libcaesium found');
}

