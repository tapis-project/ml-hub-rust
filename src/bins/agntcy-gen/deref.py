#!/usr/bin/env python3

import json
from pprint import pprint


class SpecDereferencer:
    def __init__(self, spec: dict, refs: dict, skip: list=list):
        self.spec = spec
        self.refs = refs
        self.skip = skip

    def skip_ref(self, ref):
        for skip in self.skip:
            if skip in ref:
                return True
            
        return False

    def walk_obj(self, obj):
        for k in list(obj.keys()):
            if k == "$ref":
                ref = obj[k]
                if self.skip_ref(ref):
                    continue
                obj.update(self.refs[ref])
                del obj[k]
                continue

            self.deref_value(obj[k])

    def walk_array(self, array):
        for item in array:
            self.deref_value(item)

    def deref_value(self, value):
        if type(value) == dict:
            self.walk_obj(value)

        if type(value) == list:
            self.walk_array(value)
            
    def deref(self):
        self.deref_value(self.spec)
        return self.spec

if __name__ == "__main__":
    with open("agntcy0.7.0.json", "r") as file:
        spec = json.loads(file.read())
 
    refs = {}
    for i in spec["$defs"]:
        for j in spec["$defs"][i]:
            refs[f"#/$defs/{i}/{j}"] = spec["$defs"][i][j]

    with open("agntcy0.7.0-refs.json", "w") as file:
        json.dump(refs, file, indent=4)

    derefed_spec = SpecDereferencer(spec, refs, skip=["env_var_values"]).deref()

    with open("agntcy0.7.0-derefed.json", "w") as file:
        json.dump(derefed_spec, file, indent=4)