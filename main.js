const caesium = require('./build/caesium');

let parameters = {
    jpeg: {
        quality: 20
    },
    png: {
        quality: 20,
        force_zopfli: false
    },
    webp: {
        quality: 20
    },
    gif: {
        quality: 20
    },
    keep_metadata: true,
    optimize: false,
    width: 0,
    height: 0
}

caesium.compress('samples/input.jpg', 'samples/output.jpg', parameters);