import sys
import json
import os
import re

def normalize_caption(caption: str):
    modified_caption = "".join([part.capitalize() for part in caption.replace("-", " ").replace("_", " ").replace("/", " ").split(" ")])
    modified_caption = re.sub(r"\([a-zA-Z0-9]*\)", "", modified_caption)
    
    return modified_caption

def normalize_category_caption(caption: str):
    category = normalize_caption(caption)

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
    def __init__(self, category=None, caption=None, uid=None, key: str=None, name=None, description=None):
        self.category: str = category
        self.caption: str = caption
        self.uid: int = uid
        self.key: str = key 
        self.name: str = name
        self.description: str = description

    def __repr__(self):
        return f"{self.category}::{self.caption}::{self.key}::{self.uid}::{self.name}"

spec_path = sys.argv[1]
gen_dir = sys.argv[2]
templates_dir = sys.argv[3]

spec = {}
with open(spec_path, mode="r") as file:
    spec = json.load(file)

# Produces a flat list of all the elements for a given category.
# NOTE No recursion is necessary. An element can either be created by the items
# in the attributes array or the attributes[key]["classes"] array
# NOTE This only works if there are 
elements: list[Element] = []
category = normalize_category_caption(spec["caption"])

# These 2 sets are for determining if there are duplicate captions. Once the
# elements are created, we fix the duplications
captions = set()
duplicate_captions = set()
for key, value in spec["attributes"].items():
    # Tracks the size of the
    last_captions_size = len(captions)

    if normalize_caption(value["caption"]) not in TOP_LEVEL_ELEMENTS_TO_SKIP:
        # Create the top level elements
        elements.append(
            Element(
                category=category, 
                caption=normalize_caption(value["caption"]), 
                key=key, 
                name=value["name"], 
                uid=value["uid"],
                description=value["description"]
            )
        )
    
    # Each item in the classes array is an element. Each element be either the child
    # of a top level element or one of the other classes. Only the name will changed
    # based on one of the 2 relationships above; the other information for each
    # element can be found on the element itself.
    for k in value["classes"]:
        name = value["name"]
        caption = normalize_caption(value["classes"][k]["caption"])
        # Detect duplicate sets
        captions.add(caption)
        if len(captions) == last_captions_size:
            duplicate_captions.add(caption)
        
        last_captions_size = len(captions)
        
        if value["classes"][k]["extends"] == value["name"]:
            name = f'{name}/{value["classes"][k]["name"]}'
            elements.append(
                Element(
                    category=category, 
                    caption=caption, 
                    key=k, 
                    name=name, 
                    uid=value["classes"][k]["uid"],
                    description=value["classes"][k]["description"],
                )
            )
            continue

        name = f"{name}/{value["classes"][k]["extends"]}/{value["classes"][k]["name"]}"
        elements.append(
            Element(
                category=category, 
                caption=caption, 
                key=k, 
                name=name, 
                uid=value["classes"][k]["uid"],
                description=value["classes"][k]["description"],
            )
        )

# De-duplicate element captions with the key of the element
for duplicate_caption in duplicate_captions:
    dupe_captioned_elements = list(filter(lambda e: e.caption == duplicate_caption, elements))
    for el in dupe_captioned_elements:
        el.caption = normalize_caption(el.key)

# Load base enum template
tab = "    "
with open(os.path.join(templates_dir, "enum.rs.template"), mode="r") as file:
    enum_template = file.read()

partially_hydrated_template = enum_template.replace("{{ enum_docstring }}", f'#[doc = \"{spec["description"]}\"]').replace("{{ feature_attr }}", f"#[cfg(feature = \"{category.lower()}\")]")

partially_hydrated_template = partially_hydrated_template.replace("{{ EnumName }}", category)
enum_items = ""
for element in elements:
    enum_items += f"{tab}#[doc = \"{element.description}\"]\n{tab}{element.caption},\n"
    
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
    uid_match_arms += f"{tab*3}{{{{ Category }}}}::{element.caption} => {element.uid},\n".replace("{{ Category }}", category)

name_match_arms = ""
for element in elements:
    name_match_arms += f"{tab*3}{{{{ Category }}}}::{element.caption} => \"{element.name}\",\n".replace("{{ Category }}", category)

hydrated_identify_impl_template = partially_hydrated_identify_impl_template.replace("{{ uid_match_arms }}", uid_match_arms).replace("{{ name_match_arms }}", name_match_arms)

partially_hydrated_template = partially_hydrated_template.replace("{{ identify_impl }}", hydrated_identify_impl_template)

# Enum to String impl
with open(os.path.join(templates_dir, "from_enum_to_string_impl.rs.template"), mode="r") as file:
    impl_template = file.read()

partially_hydrated_impl_template = f"{impl_template}".replace("{{ Category }}", category).replace("{{ feature_attr }}", f"#[cfg(feature = \"{category.lower()}\")]")
match_arms = ""
for element in elements:
    match_arms += f"{tab*3}{{{{ Category }}}}::{element.caption} => \"{element.name}\",\n".replace("{{ Category }}", category)

hydrated_impl_template = partially_hydrated_impl_template.replace("{{ match_items }}", match_arms)

partially_hydrated_template = partially_hydrated_template.replace("{{ enum_to_string_impl }}", hydrated_impl_template)

# Enum to u32
with open(os.path.join(templates_dir, "from_enum_to_u32_impl.rs.template"), mode="r") as file:
    impl_template = file.read()

partially_hydrated_impl_template = f"{impl_template}".replace("{{ Category }}", category).replace("{{ feature_attr }}", f"#[cfg(feature = \"{category.lower()}\")]")
match_arms = ""
for element in elements:
    match_arms += f"{tab*3}{{{{ Category }}}}::{element.caption} => {element.uid},\n".replace("{{ Category }}", category)

hydrated_impl_template = partially_hydrated_impl_template.replace("{{ match_items }}", match_arms)

partially_hydrated_template = partially_hydrated_template.replace("{{ enum_to_u32_impl }}", hydrated_impl_template)

# Combine everything
hydrated_template = partially_hydrated_template
with open(os.path.join(gen_dir, f"{category.lower()}.rs"), mode="w") as file:
    file.write(hydrated_template)



