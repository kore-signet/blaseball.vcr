from pprint import pprint
from genson import SchemaBuilder
from blaseball_mike import chronicler
from colored import fg, bg, attr
import io
import subprocess
import os.path
import re

derive_replace = re.compile(r"\[derive\(Serialize, Deserialize\)\]")


def build_types(args):
    for i, type_name in enumerate(args.types.keys()):
        print(
            f"{attr(1)}-// building schema for {type_name}! ({i+1}/{len(args.types)}) -//{attr(0)}"
        )
        out_path = os.path.join(args.conf["target"]["schemas"], f"{type_name}.rs")
        print(f"{fg(10)}-// output: {out_path}{attr(0)}")
        build_type(type_name, out_path)


def build_type(ty: str, out):
    schema = SchemaBuilder()
    i = 0
    for version in chronicler.get_versions(ty):
        i += 1
        schema.add_object(version["data"])
        if i % 100 == 0:
            print(f"{fg(211)}\t% {ty} - #{i}{attr(0)}")

    subprocess.run(
        [
            "quicktype",
            "-s",
            "schema",
            "-",
            "-l",
            "rust",
            "--visibility",
            "public",
            "-o",
            out,
        ],
        input=schema.to_json(),
        encoding="utf8",
    )

    subprocess.run(
        [
            "sed",
            "-i",
            "1,12d",
            out,
        ]
    )

    subprocess.run(
        [
            "sed",
            "-i",
            "1s/.*/use crate::UuidShell;/",
            out,
        ]
    )

    subprocess.run(
        [
            "sed",
            "-i",
            r"0,/\[derive(Serialize, Deserialize)\]/ s//[derive(Serialize, Deserialize, Clone, PartialEq, vhs_diff::Patch, vhs_diff::Diff)]/g",
            out,
        ]
    )

    subprocess.run(
        [
            "sed",
            "-i",
            r"s/\[derive(Serialize, Deserialize)\]/[derive(Serialize, Deserialize, Clone, PartialEq)]/g",
            out,
        ]
    )

    subprocess.run(
        [
            "sed",
            "-i",
            r"s/extern crate serde_derive;/use serde::{Serialize, Deserialize};/g",
            out,
        ]
    )
