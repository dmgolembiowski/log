import os
import subprocess

ROOT = os.path.abspath(os.path.dirname(__file__)).strip("easy_setup.py")+"/"
SQLITE_DATABASE_PATH = None
DEDICATED_SERVER_DSN = None
DEDICATED_SERVER_TYPE = None
DATABASE_OPTIONS = [
        "SQLite",
        "PostgreSQL",
        "EdgeDB",
        "MySQL" ]
USE_DEDICATED_SERVER = False
USER = None
PASS = None
TEMPLATES_ROOT_DEFAULT = ROOT + "/templates/"
TEMPLATES_ROOT = None
TMP_FOLDER_ROOT_DEFAULT = ROOT + "/tmp/"
TMP_FOLDER_ROOT = None
EDITOR_OPTIONS = [
        "vim",
        "vi",
        "nano",
        "emacs" ]
EDITOR = None
EDITOR_DEFAULT = "vim"

