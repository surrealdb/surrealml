from pathlib import Path

current_dir = Path(__file__).parent.joinpath("..").joinpath("surrealml").joinpath("test.py")

with open(current_dir, "w") as f:
    f.write("argh yeah")
