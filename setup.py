#!/usr/bin/env python

from setuptools import find_packages, setup

from genart import __version__

with open("README.md") as readme_file:
    readme = readme_file.read()

setup(
    author="Oliver Hofkens",
    author_email="oli.hofkens@gmail.com",
    name="genart",
    version=__version__,
    description="Generative art playground",
    long_description=readme,
    include_package_data=True,
    packages=find_packages(include=["genart"]),
    setup_requires=[],
    install_requires=[],
    test_suite="tests",
    tests_require=["tox"],
    entry_points={
        "console_scripts": ["genart=genart.main:main"],
    },
)
