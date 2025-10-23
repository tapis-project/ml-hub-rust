import sys
import json
import os
import re

def normalize_name(name: str):
    modified_name = "".join([part.capitalize() for part in name.replace("-", " ").replace("_", " ").replace("/", " ").split(" ")])
    modified_name = re.sub(r"\([a-zA-Z0-9]*\)", "", modified_name)
    modified_name = modified_name.replace("&", "And")
    
    return modified_name

def normalize_category_name(name: str):
    category = normalize_name(name)

    # Singularize the category
    if category[-3] == "ies":
        return category[:-3]

    # Singularize the category
    if category[-1] == "s":
        return category[:-1]
    
    return category

# NOTE These items seem to be duplicates in the spec. Further investigation requires. 2025/10/11
TOP_LEVEL_ELEMENTS_TO_SKIP = ["Observability", "Evaluation"]

class Element:
    def __init__(self, category=None, caption=None, uid=None, name=None, description=None):
        self.category: str = category
        self.caption: str = caption
        self.uid: int = uid
        self.name: str = name
        self.description: str = description

    def __repr__(self):
        return f"{self.category}::{self.caption}::{self.uid}::{self.name}"

spec_path = sys.argv[1]
gen_dir = sys.argv[2]
templates_dir = sys.argv[3]
taxon = sys.argv[4]

spec = {}
with open(spec_path, mode="r") as file:
    spec = json.load(file)

# Produces a flat list of all the elements for a given category.
elements: list[Element] = []
category = normalize_category_name(taxon)

taxon_descriptions = {
    "domains": "Distinct fields of application and knowledge areas",
    "skills": "Distinct abilities",
    "modules": "Module sets of application and knowledge areas",
}

for value in spec:
    elements.append(
        Element(
            category=category, 
            caption=value["caption"], 
            name=value["name"], 
            uid=value["uid"],
            description=value["description"],
        )
    )

# Load base enum template
tab = "    "
with open(os.path.join(templates_dir, "enum.rs.template"), mode="r") as file:
    enum_template = file.read()

partially_hydrated_template = enum_template.replace("{{ enum_docstring }}", f'#[doc = \"{taxon_descriptions.get(taxon, taxon)}\"]').replace("{{ feature_attr }}", f"#[cfg(feature = \"{category.lower()}\")]")

partially_hydrated_template = partially_hydrated_template.replace("{{ EnumName }}", category)
enum_items = ""
for element in elements:
    enum_items += f"{tab}#[doc = \"{element.description}\"]\n{tab}{normalize_name(element.name)},\n"
    
partially_hydrated_template = partially_hydrated_template.replace("{{ enum_items }}", enum_items)

# Identify trait
with open(os.path.join(templates_dir, "identify_trait.rs.template"), mode="r") as file:
    identify_trait_template = file.read()

hydrated_identify_trait_template = identify_trait_template.replace("{{ feature_attr }}", f"#[cfg(all(feature = \"{category.lower()}\", feature = \"identify\"))]")

partially_hydrated_template = partially_hydrated_template.replace("{{ identify_trait }}", hydrated_identify_trait_template)

# Identify impl
with open(os.path.join(templates_dir, "identify_impl.rs.template"), mode="r") as file:
    identify_impl_template = file.read()

partially_hydrated_identify_impl_template = identify_impl_template.replace("{{ Category }}", category).replace("{{ feature_attr }}", f"#[cfg(all(feature = \"{category.lower()}\", feature = \"identify\"))]")

uid_match_arms = ""
for element in elements:
    uid_match_arms += f"{tab*3}{{{{ Category }}}}::{normalize_name(element.name)} => {element.uid},\n".replace("{{ Category }}", category)

name_match_arms = ""
for element in elements:
    name_match_arms += f"{tab*3}{{{{ Category }}}}::{normalize_name(element.name)} => \"{element.name}\",\n".replace("{{ Category }}", category)

hydrated_identify_impl_template = partially_hydrated_identify_impl_template.replace("{{ uid_match_arms }}", uid_match_arms).replace("{{ name_match_arms }}", name_match_arms)

partially_hydrated_template = partially_hydrated_template.replace("{{ identify_impl }}", hydrated_identify_impl_template)

# Enum to String impl
with open(os.path.join(templates_dir, "from_enum_to_string_impl.rs.template"), mode="r") as file:
    impl_template = file.read()

partially_hydrated_impl_template = f"{impl_template}".replace("{{ Category }}", category).replace("{{ feature_attr }}", f"#[cfg(feature = \"{category.lower()}\")]")
match_arms = ""
for element in elements:
    match_arms += f"{tab*3}{{{{ Category }}}}::{normalize_name(element.name)} => \"{element.name}\",\n".replace("{{ Category }}", category)

hydrated_impl_template = partially_hydrated_impl_template.replace("{{ match_items }}", match_arms)

partially_hydrated_template = partially_hydrated_template.replace("{{ enum_to_string_impl }}", hydrated_impl_template)

# Enum to u32
with open(os.path.join(templates_dir, "from_enum_to_u32_impl.rs.template"), mode="r") as file:
    impl_template = file.read()

partially_hydrated_impl_template = f"{impl_template}".replace("{{ Category }}", category).replace("{{ feature_attr }}", f"#[cfg(feature = \"{category.lower()}\")]")
match_arms = ""
for element in elements:
    match_arms += f"{tab*3}{{{{ Category }}}}::{normalize_name(element.name)} => {element.uid},\n".replace("{{ Category }}", category)

hydrated_impl_template = partially_hydrated_impl_template.replace("{{ match_items }}", match_arms)

partially_hydrated_template = partially_hydrated_template.replace("{{ enum_to_u32_impl }}", hydrated_impl_template)

# Combine everything
hydrated_template = partially_hydrated_template
with open(os.path.join(gen_dir, f"{category.lower()}.rs"), mode="w") as file:
    file.write(hydrated_template)



