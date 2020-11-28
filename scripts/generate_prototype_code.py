import json


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

type_aliases = {
    "ItemStackIndex": "u16",
    "ItemCountType": "u32",
    "FileName": "PathBuf", # Or use something that doesn't need so much space, like interned paths, i.e. Arc<PathBuf>
}


with open("prototypes.json") as f:
    prototypes = json.load(f)


