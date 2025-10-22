import sys
import json
import os
import re

class Task:
    def __init__(self, key=None, id_=None, description=None):
        self.key: str = key
        self.id: str = id_
        self.name: str = "".join([part.capitalize() for part in key.split("-")])
        self.description: str = description
       
    def __repr__(self):
        return f"{self.name}::{self.key}::{self.id}"

def hydrate_templates(templates_dir: str, gen_dir: str, tasks: list[Task]):
    # Load base enum template
    tab = "    "
    with open(os.path.join(templates_dir, "enum.rs.template"), mode="r") as file:
        enum_template = file.read()

    partially_hydrated_template = enum_template.replace("{{ enum_docstring }}", f'#[doc = \"An enum of all task types available on Huggingface\"]')

    partially_hydrated_template = partially_hydrated_template.replace("{{ EnumName }}", "Task")
    enum_items = ""
    for task in tasks:
        enum_items += f"{tab}#[doc = \"{task.description}\"]\n{tab}{task.name},\n"
        
    partially_hydrated_template = partially_hydrated_template.replace("{{ enum_items }}", enum_items)

    # Combine everything
    hydrated_template = partially_hydrated_template
    with open(os.path.join(gen_dir, f"task.rs"), mode="w") as file:
        file.write(hydrated_template)


spec_path = sys.argv[1]
gen_dir = sys.argv[2]
templates_dir = sys.argv[3]

task_spec = {}
with open(spec_path, mode="r") as file:
    task_spec = json.load(file)

tasks: list[Task] = []

for key, value in task_spec.items():
    tasks.append(Task(key=key, id_=value.get("id"), description=value.get("summary")))

hydrate_templates(templates_dir, gen_dir, tasks)


