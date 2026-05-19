const utils = require('../../utils');

console.log("MLHub Models API transform");
try {
    // Copy models spec  without transforms
    utils.copy('models');
} catch (error) {
    console.error(error);
}