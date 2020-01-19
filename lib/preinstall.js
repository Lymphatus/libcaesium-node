const {execSync} = require('child_process');
const log = require('logalot');

const MIN_CMAKE_VERSION = '2.8.12';
const MIN_NASM_VERSION = '2.14.02';

function checkCMakeVersion() {
  try {
    const stdout = execSync('cmake --version').toString();
    let matches = stdout.match(/cmake version (\d+\.\d+\.\d+)/);
    if (!matches || !matches.length) {
      log.error('Cannot detect CMake version.');
      return false;
    }
    const installed_cmake_version = matches[1];
    if (installed_cmake_version < MIN_CMAKE_VERSION) {
      log.error('Your CMake version is not compatible with this module. Please install CMake >= ' + MIN_CMAKE_VERSION);
      return false;
    }

    log.success('CMake version ' + installed_cmake_version + ' detected.');
    return true;
  } catch (e) {
    log.error('CMake was not detected.');
    return false;
  }
}

function checkNASMVersion() {
  try {
    const stdout = execSync('nasm --version').toString();
    let matches = stdout.match(/NASM version (\d+\.\d+\.\d+)/);
    if (!matches || !matches.length) {
      log.error('Cannot detect NASM version.');
      return false;
    }
    const installed_nasm_version = matches[1];
    if (installed_nasm_version < MIN_NASM_VERSION) {
      log.error('Your NASM version is not compatible with this module. Please install NASM >= ' + MIN_NASM_VERSION);
      return false;
    }

    log.success('NASM version ' + installed_nasm_version + ' detected.');
    return true;
  } catch (e) {
    log.error('NASM was not detected.');
    return false;
  }
}

return checkCMakeVersion() && checkNASMVersion();
