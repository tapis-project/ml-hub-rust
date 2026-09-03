const fs = require('fs');
const yaml = require('js-yaml');

/**
 * Copy a service's yml file directly to the transformed_openapi directory
 * if no transformations are required 
 * @param   {string}    service service name
 */
const copy = (service) => {
    fs.copyFileSync(`./services/${service}/spec.yml`, `./services/${service}/transformed_spec.yml`);
}

/**
 * Load a service's yml file and return it as a 
 * @param   {string}    service service name
 * @return  {any}               object representation of yml
 */
const read = (service) => {
    return yaml.load(fs.readFileSync(`./services/${service}/spec.yml`, 'utf8'));
}

const write = (doc, service) => {
    fs.writeFileSync(`./services/${service}/transformed_spec.yml`, yaml.dump(doc));
}

module.exports.copy = copy;
module.exports.read = read;
module.exports.write = write;