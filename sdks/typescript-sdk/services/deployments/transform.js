const utils = require('../../utils');

console.log("MLHub Deployments API transform");
try {
    // Copy deployments spec  without transforms
    utils.copy('deployments');
} catch (error) {
    console.error(error);
}