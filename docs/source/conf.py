# Configuration file for the Sphinx documentation builder.
#
# For the full list of built-in configuration values, see the documentation:
# https://www.sphinx-doc.org/en/master/usage/configuration.html

# -- Project information -----------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#project-information

project = 'enlil'
copyright = '2026, Kevan Anderson'
author = 'Kevan Anderson'
release = '0.0.1'

# -- General configuration ---------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#general-configuration

extensions = [
    'sphinxcontrib_rust',
    'myst_parser',

]

templates_path = ['_templates'] 
exclude_patterns = []



# -- Options for HTML output -------------------------------------------------
# https://www.sphinx-doc.org/en/master/usage/configuration.html#options-for-html-output

html_theme = 'alabaster'
html_static_path = [] # ['_static'] only needed if there is content in the _static folder


# -- Options for rust -------------------------------------------------
# Configuration for sphinxcontrib-rust
rust_crates = {
    "enlil": "..", # Path to your crate's source directory relative to conf.py
}
# Define where the generated documentation files will be placed
rust_doc_dir = "source/crates" # This directory should be added to your .gitignore