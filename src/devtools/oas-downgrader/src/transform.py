import re
import sys
import os

def downgrade(spec):
    modified_spec = spec
    for t in ["string", "integer", "array", "boolean"]:
        modified_spec = downgrade_nullable_types(modified_spec, t)

    # Transform the nullable oneOfs to allOfs + nullable: true
    oneOf = r"oneOf:\n(\s+)\- type: 'null'\n\s+\- (\$ref: '#/components/schemas/HardwareRequirements')"
    allOf = r"nullable: true\n          allOf:\n\1- \2"
    modified_spec = re.sub(oneOf, allOf, modified_spec)

    # Remove all instances of the 'propertyNames' prop of additionalProperties object
    modified_spec = remove_additionalProperties_propertyNames_prop(modified_spec)

    return modified_spec

    
def downgrade_nullable_types(spec, _type):
    spec = re.sub(fr"type:\n\s{{12}}- {_type}", f"type: {_type}", spec, flags=re.MULTILINE)
    spec = spec.replace("  - 'null'", "nullable: true")

    return spec

def remove_additionalProperties_propertyNames_prop(spec):
    spec = re.sub(r"\s{12}propertyNames:\n\s{14}type:\sstring\n", "", spec, flags=re.MULTILINE)

    return spec

if __name__ == "__main__":
    source = sys.argv[1]
    destination = sys.argv[2]

    spec = ""
    with open(source, mode="r") as file:
        spec = file.read()
    
    modified_spec = downgrade(spec)

    destination_dir = "/".join(destination.split("/")[:-1])

    os.makedirs(destination_dir, exist_ok=True)

    with open(destination, mode="w") as file:
        file.write(modified_spec.replace("openapi: 3.1.0", "openapi: 3.0.3"))