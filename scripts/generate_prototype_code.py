import json
import re


primitive_types = {
    "bool": "bool",
    "double": "f64",
    "float": "f32",
    "int16": "i16",
    "int32": "i32",
    "int8": "i8",
    "string": "String",
    "uint16": "u16",
    "uint32": "u32",
    "uint8": "u8",
    "vector": "Vector2<f32>",
}

# TODO: We'll define those in code
type_aliases = {
    "ItemStackIndex": "u16",
    "ItemCountType": "u32",
    "FileName": "PathBuf",  # Or use something that doesn't need so much space, like interned paths, i.e. Arc<PathBuf>
}


def patch_field_name(name):
    if name == "type":
        name = "r#type"
    return name

def gen_type(ty, optional):
    if type(ty) == str:
        ty = primitive_types.get(ty, ty)
    else:
        if "tuple" in ty:
            assert ty["tuple"]["n"] == 2
            "[{}; {}]".format(ty["tuple"]["of"], ty["tuple"]["n"])
        elif "or" in ty:
            ty = "(/*TODO*/)"
        elif "table_of" in ty:
            ty = "Vec<{}>".format(ty["table_of"])
        else:
            assert False
    if optional:
        ty = "Option<{}>".format(ty)
    return ty


def gen_field(field):
    return """    /// {}
    {}: {},    
""".format(field["comment"], patch_field_name(field["name"]), gen_type(field["type"], field.get("optional", False)))


def gen_struct(proto, fields):
    return """#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct {} {{
{}
}}

impl Prototype for {} {{
    const TYPE: Option<&'static str> = Some("{}");
}}

""".format(proto["class_name"], fields, proto["class_name"], proto.get("name", "None"))


with open("prototypes.json") as f:
    prototypes = json.load(f)

mod_rs = []
types = set()

for proto in prototypes:
    for prop in proto["properties"]:
        if type(prop["type"]) == str:
            types.add(prop["type"])

    props = "\n".join([gen_field(field) for field in proto["properties"]])
    struct = gen_struct(proto, props)
    mod_name = re.sub(r'(?<!^)(?=[A-Z])', '_', proto["class_name"]).lower()

    mod_rs.append(mod_name)

    with open("prototypes/{}.rs".format(mod_name), "wt") as f:
        f.write("""use serde::{{Serialize, Deserialize}};
        
use super::{{Prototype, type_stubs::*}};
        
{}
""".format(struct))


with open("prototypes/mod.rs", "wt") as f:
    f.write("".join(["// pub mod {};\n".format(mod) for mod in mod_rs]))
    f.write("""
pub mod type_stubs;
    
pub trait Prototype {
    const TYPE: Option<&'static str>;
}    
    """)

with open("prototypes/type_stubs.rs", "wt") as f:
    f.write("use serde::{Serialize, Deserialize};\n\n")
    for ty in list(sorted(types)):
        if ty not in primitive_types:
            f.write("""#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct {}();

""".format(ty))
