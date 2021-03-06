import sys
import platform

from setuptools import setup
from setuptools_rust import RustExtension


def get_py_version_cfgs():
    # For now each Cfg Py_3_X flag is interpreted as "at least 3.X"
    version = sys.version_info[0:2]
    py3_min = 6
    out_cfg = []
    for minor in range(py3_min, version[1] + 1):
        out_cfg.append("--cfg=Py_3_%d" % minor)

    if platform.python_implementation() == "PyPy":
        out_cfg.append("--cfg=PyPy")

    return out_cfg


def make_rust_extension(module_name):
    return RustExtension(
        module_name, "Cargo.toml", rustc_flags=get_py_version_cfgs(), debug=True
    )


setup(
    name="cc-sandbox",
    version="0.1.0",
    classifiers=[
        "License :: OSI Approved :: MIT License",
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "Programming Language :: Python",
        "Programming Language :: Rust",
        "Operating System :: POSIX",
        "Operating System :: MacOS :: MacOS X",
    ],
    packages=["cc_sandbox"],
    rust_extensions=[
        make_rust_extension("cc_sandbox.cc_sandbox"),
        make_rust_extension("cc_sandbox.objstore"),
    ],
    include_package_data=True,
    zip_safe=False,
)
