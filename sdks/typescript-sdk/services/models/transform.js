const utils = require('../../utils');

console.log("MLHub Models API transform");
try {
    // Copy workflows spec  without transforms
    utils.copy('models');
} catch (error) {
    console.error(error);
}