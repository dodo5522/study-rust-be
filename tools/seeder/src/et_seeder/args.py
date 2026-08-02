import argparse
from dataclasses import dataclass, field
from pathlib import Path


@dataclass
class SeederOptions:
    base_url: str
    post_master: bool = False
    csv_files: list[Path] = field(default_factory=list)


def parse_args(argv: list[str] | None = None) -> SeederOptions:
    parser = argparse.ArgumentParser(
        prog="seeder",
        description="Seed energy generation master data and measurements.",
    )
    parser.add_argument(
        "-b",
        "--base-url",
        action="store",
        type=str,
        default="http://localhost:8000/generation",
        help="base URL to post master data and measurements",
    )
    parser.add_argument(
        "-m",
        "--post-master",
        action="store_true",
        help="post master data before measurements",
    )
    parser.add_argument(
        "csv_files",
        nargs="+",
        type=Path,
        help="CSV files to import as measurement data",
    )

    namespace = parser.parse_args(argv)
    return SeederOptions(
        base_url=namespace.base_url,
        post_master=namespace.post_master,
        csv_files=namespace.csv_files,
    )
