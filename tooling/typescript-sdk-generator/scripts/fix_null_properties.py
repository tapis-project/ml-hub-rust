import sys
import json


def fix_null_properties(data):
    if isinstance(data, dict):
        # Look for the broken oneOf structure
        if 'oneOf' in data and len(data['oneOf']) == 2:
            has_null = any(item.get('type') == 'null' for item in data['oneOf'])
            ref_item = next((item for item in data['oneOf'] if '$ref' in item), None)
            
            if has_null and ref_item:
                # Convert it to clean OpenAPI 3.0 syntax
                return {'$ref': ref_item['$ref'], 'nullable': True}
        
        # Recursively scan the rest of the dictionary
        return {k: fix_null_properties(v) for k, v in data.items()}
    elif isinstance(data, list):
        return [fix_null_properties(item) for item in data]
    return data

# Load your downgraded spec
source = sys.argv[1]
with open(source, 'r') as f:
    spec = json.load(f)

# Process and save the fixed spec
fixed_spec = fix_null_properties(spec)
destination = sys.argv[2]
with open(destination, 'w') as f:
    json.dump(fixed_spec, f, sort_keys=False)