import os
import sys
sys.path.insert(0, os.path.abspath('../../'))

project = 'Mailer API'
copyright = '2025, Votre Nom'
author = 'Votre Nom'
release = '0.1.0'

extensions = [
    'sphinx.ext.autodoc',
    'sphinx.ext.viewcode',
    'sphinx.ext.napoleon',
    'myst_parser',
]

templates_path = ['_templates']
exclude_patterns = []

html_theme = 'sphinx_rtd_theme'
html_static_path = ['_static']

# Configuration pour la langue française
language = 'fr'
locale_dirs = ['locale/']
gettext_compact = False 