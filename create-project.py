#!/usr/bin/env python3

import os
import subprocess
import shutil


ABS_PATH = os.path.dirname(os.path.realpath(__file__))

print("Welcome to GTK Rust Template")
name = input("Name (e.g. My Awesome App): ")
project_name = input("Project Name (e.g. my-awesome-app): ")
app_id = input(
    "Application ID (see: https://developer.gnome.org/ChooseApplicationID/): "
)
author = input("Author: ")
update_contact = input("Email: ")
project_repo = input("Github/Gitlab repository: ").rstrip("/")
is_gtk4 = input("Use gtk4 [Y/n]: ").lower() != "n"

source_dir = "gtk4" if is_gtk4 else "gtk3"
app_path = "/" + "/".join(app_id.split(".")) + "/"
project_dir = os.path.join(ABS_PATH, project_name)

CURRENT_APP_ID = "com.belmoussaoui.GtkRustTemplate"
CURRENT_PROJECT_NAME = "gtk-rust-template"
CURRENT_NAME = "GTK Rust Template"
CURRENT_AUTHOR = "Bilal Elmoussaoui"
CURRENT_EMAIL = "bil.elmoussaoui@gmail.com"
CURRENT_APP_PATH = "/com/belmoussaoui/GtkRustTemplate/"
CURRENT_PROJECT_REPO = "https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template"

FILES = [
    f"{source_dir}/com.belmoussaoui.GtkRustTemplate.Devel.json",
    "data/icons/com.belmoussaoui.GtkRustTemplate-symbolic.svg",
    "data/icons/com.belmoussaoui.GtkRustTemplate.svg",
    "data/icons/com.belmoussaoui.GtkRustTemplate.Devel.svg",
    "data/com.belmoussaoui.GtkRustTemplate.desktop.in.in",
    "data/com.belmoussaoui.GtkRustTemplate.gschema.xml.in",
    "data/com.belmoussaoui.GtkRustTemplate.metainfo.xml.in.in",
    "data/resources.gresource.xml",
    "po/POTFILES.in",
    f"{source_dir}/src/application.rs",
    f"{source_dir}/src/main.rs",
    f"{source_dir}/src/window.rs",
    f"{source_dir}/Cargo.toml",
    f"{source_dir}/meson.build",
    "meson_options.txt",
    "ci-gitlab.yml",
    "ci-github.yml",
]

TO_RENAME = [
    f"{source_dir}/com.belmoussaoui.GtkRustTemplate.Devel.json",
    "data/icons/com.belmoussaoui.GtkRustTemplate-symbolic.svg",
    "data/icons/com.belmoussaoui.GtkRustTemplate.svg",
    "data/icons/com.belmoussaoui.GtkRustTemplate.Devel.svg",
    "data/com.belmoussaoui.GtkRustTemplate.desktop.in.in",
    "data/com.belmoussaoui.GtkRustTemplate.gschema.xml.in",
    "data/com.belmoussaoui.GtkRustTemplate.metainfo.xml.in.in",
]

TO_RENAME_AND_MOVE = [
    (
        f"{project_dir}/{source_dir}/{app_id}.Devel.json",
        f"{project_dir}/build-aux/{app_id}.Devel.json",
    ),
    (
        f"{project_dir}/{source_dir}/src",
        f"{project_dir}/src",
    ),
    (
        f"{project_dir}/{source_dir}/Cargo.toml",
        f"{project_dir}/Cargo.toml",
    ),
    (
        f"{project_dir}/{source_dir}/meson.build",
        f"{project_dir}/meson.build",
    ),
    (
        f"{project_dir}/{source_dir}/data/resources/ui/shortcuts.ui",
        f"{project_dir}/data/resources/ui/shortcuts.ui",
    ),
    (
        f"{project_dir}/{source_dir}/data/resources/ui/window.ui",
        f"{project_dir}/data/resources/ui/window.ui",
    ),
    (
        f"{project_dir}/ci-gitlab.yml",
        f"{project_dir}/.gitlab-ci.yml",
    ),
    (
        f"{project_dir}/ci-github.yml",
        f"{project_dir}/.github/workflows/ci.yml",
    ),
]


TO_REMOVE = ["gtk3", "gtk4"]

if os.path.isdir(project_dir):
    wanna_remove = ""
    while wanna_remove not in ["y", "n"]:
        wanna_remove = input("Project already exists, do you want to remove it? [Y/n]").lower()

    if wanna_remove != "n":
        shutil.rmtree(project_dir)
    else:
        exit()

folders_to_move = [
    "build-aux",
    "data",
    "gtk3",
    "gtk4",
    "hooks",
    "po",
    "meson_options.txt",
    ".editorconfig",
    ".gitignore",
    "ci-github.yml",
    "ci-gitlab.yml",
    "README.md",
]

os.makedirs(project_dir + "/data/resources/ui", exist_ok=True)
os.makedirs(project_dir + "/.github/workflows", exist_ok=True)

for folder in folders_to_move:
    folder_path = f"./{folder}"
    if os.path.isdir(folder_path):
        shutil.copytree(folder_path, f"./{project_name}/{folder}", dirs_exist_ok=True)
    else:
        shutil.copyfile(folder_path, f"./{project_name}/{folder}")

for file in FILES:
    current_path = os.path.join(project_dir, file)

    with open(current_path, "r") as handle:
        content = handle.read()
        content = content.replace(CURRENT_APP_ID, app_id)
        content = content.replace(CURRENT_APP_PATH, app_path)
        content = content.replace(CURRENT_PROJECT_REPO, project_repo)
        content = content.replace(CURRENT_PROJECT_NAME, project_name)
        content = content.replace(CURRENT_NAME, name)
        content = content.replace(CURRENT_AUTHOR, author)
        content = content.replace(CURRENT_EMAIL, update_contact)

    with open(current_path, "w") as handle:
        handle.write(content)

for file in TO_RENAME:
    current_path = os.path.join(project_dir, file)
    new_path = os.path.join(project_dir, file.replace(CURRENT_APP_ID, app_id))
    shutil.move(current_path, new_path)

for (src, dest) in TO_RENAME_AND_MOVE:
    shutil.move(src, dest)


for file in TO_REMOVE:
    current_path = os.path.join(project_dir, file)
    if os.path.isdir(current_path):
        shutil.rmtree(current_path)
    else:
        os.remove(current_path)

subprocess.call(["git", "init"], cwd=project_dir)
# Add all files and commit them
subprocess.call(["git", "add", "-A"], cwd=project_dir)
subprocess.call(["git", "commit", "-m", "Init with GTK Rust Template"], cwd=project_dir)

subprocess.call(
    ["git", "remote", "add", "origin", f"{project_repo}.git"], cwd=project_dir
)
