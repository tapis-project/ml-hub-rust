const fs = require('fs');
const yaml = require('js-yaml');

/**
 * Alter any methods of any paths matching a pattern to have only one tag
 * @param   {string}    doc     the yml object
 * @param   {string}    pattern a search string for paths
 * @param   {string}    tag     the tag to set for each method in matching paths
 * @return  {any}               a new object representation of yml
 */
const setTag = (doc, pattern, tag) => {
    const result = { ...doc }
    Object.keys(result.paths).filter(path => path.startsWith(pattern)).forEach(
        path => {
            const pathObj = result.paths[path];
            Object.keys(pathObj).forEach(
                method => {
                    pathObj[method].tags = [ tag ]
                }
            )
        }
    )
    return result;
}

/**
 * Recursively traverse a yml document and apply a transformation function
 * @param {any}     doc        Object to recurse through
 * @param {func}    transform  A transformation function to apply in place
 */
const recurse = (doc, transform) => {
    if (doc === null) {
        return;
    }
    // Apply transform function
    transform(doc);
    if (Array.isArray(doc)) {
        doc.forEach(
            element => recurse(element, transform)
        )
    } else if (typeof doc === "object") {
        Object.keys(doc).forEach(
            key => recurse(doc[key], transform)
        )
    }
}

/**
 * Replaces schema names, including refs
 * @param   {any}       doc         the yml object
 * @param   {string}    oldSchema   the original schema name
 * @param   {string}    newSchema   the new name for the schema
 */
const renameSchema = (doc, oldSchema, newSchema) => {
    const result = { ...doc }
    const transform = (obj) => {
        if (obj instanceof Array || !(obj instanceof Object)) {
            return;
        }
        Object.keys(obj).forEach(
            key => {
                if (key === '$ref') {
                    if (obj[key] === `#/components/schemas/${oldSchema}`) {
                        obj[key] = `#/components/schemas/${newSchema}`;
                    }
                }
            }
        )
    }
    recurse(result, transform);
    result.components.schemas[newSchema] = { ...result.components.schemas[oldSchema] }
    delete result.components.schemas[oldSchema];
    return result;
}

/**
 * Prepend all paths with a prefix
 * @param {any} doc 
 * @param {string} prefix 
 */
const prependPaths = (doc, prefix) => {
    const result = { ...doc };
    Object.keys(result.paths).forEach(
        path => {
            result.paths[`${prefix}${path}`] = { ...result.paths[path] };
            delete result.paths[path]
        }
    )
    return result;
}

/**
 * Expand BasicResponse discriminators into explicit type
 * @param {any} doc 
 */
const expandBasicResponses = (doc) => {
    const result = { ...doc }
    Object.keys(result.paths).forEach(
        path => {
            const pathRef = result.paths[path];
            Object.keys(pathRef).forEach(
                method => {
                    const methodRef = pathRef[method];
                    Object.keys(methodRef.responses).forEach(
                        status => {
                            console.log({status})
                            const statusRef = methodRef.responses[status];
                            console.log({statusRef})
                            const schemaRef = statusRef['content'][Object.keys(statusRef.content)[0]]['schema'];
                            console.log({schemaRef})
                            if ('allOf' in schemaRef &&
                                '$ref' in schemaRef['allOf'][0] &&
                                schemaRef['allOf'][0]["$ref"] === '#/components/schemas/BasicResponse') {
                                const spliced = methodRef.operationId.split('_').reduce(
                                    (accumulator, current) => {
                                        return accumulator
                                            + current.charAt(0).toUpperCase() 
                                            + current.slice(1);
                                    }
                                )
                                const newSchema = "Resp" + spliced.charAt(0).toUpperCase() + spliced.slice(1);
                                result.components.schemas[newSchema] = JSON.parse(JSON.stringify(result.components.schemas.BasicResponse))
                                result.components.schemas[newSchema].properties = {
                                    ...result.components.schemas[newSchema].properties,
                                    ...schemaRef.properties
                                }
                                statusRef['content']['application/json']['schema'] = {
                                    "$ref": `#/components/schemas/${newSchema}`
                                }
                            }
                            
                        }
                    )
                }
            )
        }
    )
    return result;
}

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

module.exports.setTag = setTag;
module.exports.copy = copy;
module.exports.read = read;
module.exports.write = write;
module.exports.renameSchema = renameSchema;
module.exports.prependPaths = prependPaths;
module.exports.expandBasicResponses = expandBasicResponses;