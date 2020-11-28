import json

from bs4 import BeautifulSoup
from pprintpp import pprint



def strip_if_starts_with(s, prefix):
    if s.startswith(prefix):
        return s[len(prefix):]
    else:
        return s


def strip_prototype_prefix(s):
    return strip_if_starts_with(s, "Prototype/")


def parse_type(parts, inner=False):
    if parts[0] == "Array of":
        parts = ["array", "of"] + parts[1:]

    if parts[0].lower() == "table" or parts[0].lower() == "array":
        if len(parts) > 1:
            if parts[1].lower() == "of two":
                ty = {"tuple": {"of": parts[2], "n": 2}}
                parts = parts[3:]
            elif parts[1].lower() == "of one or two":
                ty = {"or": [
                    {"tuple": {"of": parts[2], "n": 1}},
                    {"tuple": {"of": parts[2], "n": 2}}
                ]}
            else:
                assert parts[1].lower() == "of" or parts[1].lower() == "(array) of"
                ty = {"table_of": parse_type(parts[2:], True)}
            parts = parts[3:]
        else:
            ty = {"table_of": None}
            parts = parts[1:]
    else:
        ty = parts[0]
        parts = parts[1:]

    if len(parts) > 0 and parts[0].lower() == "or" and not inner:
        or_ty = parse_type(parts[1:], True)
        ty = {"or": [ty, or_ty]}

    return ty


def patch_prototype(proto):
    return proto


with open("Prototype_overview.html") as f:
    soup = BeautifulSoup(f.read(), "html.parser")

prototype_toc = soup.find("table", attrs={"class": "prototype-toc"})
prototype = None
prototypes = []

for tr in prototype_toc.find_all("tr"):
    section_title = tr.find("td", attrs={"class": "prototype-toc-section-title"})

    if section_title:
        if prototype is not None:
            prototypes.append(patch_prototype(prototype))
            prototype = None

        parts = list(section_title.stripped_strings)
        prototype = {
            "class_name": strip_prototype_prefix(parts[0]),
            "properties": [],
        }
        if parts[1].lower() != "abstract":
            prototype["name"] = parts[1]
        if len(parts) > 3 and parts[2].lower() == "extends":
            prototype["extends"] = strip_prototype_prefix(parts[3])
        assert len(parts) <= 4

    else:
        comment = " ".join(tr.stripped_strings)
        item_name = tr.find("td", attrs={"class": "prototype-toc-item-name"})
        item_info = tr.find("td", attrs={"class": "prototype-toc-item-info"})
        if item_name is not None and item_info is not None:
            item_info = list(item_info.stripped_strings)
            prop = {"name": list(item_name.stripped_strings)[0], "comment": comment}
            if item_info[-1].lower() == "(optional)":
                prop["optional"] = True
                item_info = item_info[:-1]
            prop["type"] = parse_type(item_info)

            if prop["type"] == "IconSpecification":
                prop["name"] = "icon_spec"
                prop["transparent"] = True

            if "," in prop["name"]:
                print(tr)
                print(prop)
                assert False

            prototype["properties"].append(prop)

if prototype is not None:
    prototypes.append(patch_prototype(prototype))

pprint(prototypes)

with open("prototypes.json", "wt") as f:
    json.dump(prototypes, f, sort_keys=True, indent=2)


types = set()
for proto in prototypes:
    for prop in proto["properties"]:
        if type(prop["type"]) == str:
            types.add(prop["type"])
types = list(sorted(types))
with open("types.txt", "wt") as f:
    for t in types:
        print(t, file = f)