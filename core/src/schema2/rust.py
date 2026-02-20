#!/usr/bin/env python3
import subprocess
from pathlib import Path
import re, os, sys, json

from pathlib import Path

def snake_to_camel(snake: str) -> str:
    """Convert snake_case string to CamelCase."""
    return ''.join(word.capitalize() for word in snake.split('_'))

def camel_to_snake(name: str) -> str:
    """Convert CamelCase to snake_case."""
    s1 = re.sub('(.)([A-Z][a-z]+)', r'\1_\2', name)
    return re.sub('([a-z0-9])([A-Z])', r'\1_\2', s1).lower()




# # folder with JSON schemas
# schema_dir = Path(".")
#
# modules = []
# for f in schema_dir.glob("*.rs"):
#     if f.name == "mod.rs":
#         continue
#     type_name = f.stem
#     rust_struct = snake_to_camel(type_name)
#     modules.append((type_name, rust_struct))
#
# with open("mod.rs", "w", encoding="utf-8") as file:
#     print("#![allow(unused_imports)]", file=file)
#     print("use serde::{Serialize, Deserialize};\n", file=file)
#     # declare modules and re-export types
#     for type_name, rust_struct in modules:
#         print(f"pub mod {type_name};", file=file)
#         print(f"pub use {type_name}::{rust_struct};\n", file=file)
#
#     # generate JournalEvent enum
#     print("#[derive(Debug, Serialize, Deserialize)]", file=file)
#     print('#[serde(tag = "event")]', file=file)
#     print("pub enum JournalEvent {", file=file)
#     for _, rust_struct in modules:
#         print(f"    {rust_struct}({rust_struct}),", file=file)
#     print("}", file=file)


# folder with JSON schemas
schema_dir = Path(".")
out_dir = Path(".")
out_dir.mkdir(parents=True, exist_ok=True)

failures = []

files = schema_dir.glob("*.json")

# for path in files:
#     name = path.stem  # filename without extension

#     with open(path, "r", encoding="utf-8") as f:
#         schema = json.load(f)

#     wrapped = {
#         "$schema": schema.get("$schema", "https://json-schema.org/draft/2020-12/schema"),
#         "$defs": {
#             name: {k: v for k, v in schema.items() if k not in ("$schema")}
#         },
#         "$ref": f"#/$defs/{name}"
#     }


#     with open(f"wrapped/{name}.json", "w", encoding="utf-8") as f:
#         json.dump(wrapped, f, indent=2)

# for json_file in schema_dir.glob("*.json"):
#     # filename without extension = Rust type name
#     type_name = json_file.stem
#     rust_filename = camel_to_snake(type_name) + ".rs"
#     rust_path = out_dir / rust_filename

#     print(f"Generating Rust for {json_file} → {rust_path}")

#     # run cargo typify CLI
#     try:
#         subprocess.run([
#             "cargo", "typify", "-B",
#             "--output", str(rust_path),
#             str(json_file)
#         ], check=True)
#         os.remove(json_file)
#     except:
#         failures.append(json_file)

mod_file = schema_dir / "mod.rs"

modules = []

# Process each generated Rust file
for f in schema_dir.glob("*.rs"):
    if f.name == "mod.rs":
        continue

    type_name = f.stem
    rust_struct = snake_to_camel(type_name)
    modules.append((type_name, rust_struct))

    # Read original file
    lines = f.read_text(encoding="utf-8").splitlines()
    new_lines = []

    for line in lines:
        if "pub event: ::std::string::String" in line:
            new_lines.append(f"    pub event: ::std::String,")
        else:
            new_lines.append(line)

    # Append impl block for default event
    new_lines.append(f"\nimpl {rust_struct} {{")
    new_lines.append(f"    pub fn event_value() -> String {{")
    new_lines.append(f"        \"{rust_struct}\".to_string()")
    new_lines.append(f"    }}")
    new_lines.append(f"}}\n")

    # Write back
    f.write_text("\n".join(new_lines) + "\n", encoding="utf-8")

# Write mod.rs
with open(mod_file, "w", encoding="utf-8") as file:
    print("#![allow(unused_imports)]", file=file)
    print("use serde::{Serialize, Deserialize};\n", file=file)

    for type_name, rust_struct in modules:
        print(f"pub mod {type_name};", file=file)
        print(f"pub use {type_name}::{rust_struct};\n", file=file)

    # Generate tagged enum
    print("#[derive(Debug, Serialize, Deserialize)]", file=file)
    print('#[serde(tag = "event")]', file=file)
    print("pub enum JournalEvent {", file=file)
    for _, rust_struct in modules:
        print(f"    {rust_struct}({rust_struct}),", file=file)
    print("}", file=file)

# print(failures)