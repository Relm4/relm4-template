#!/usr/bin/env python3

import os
import sys
import subprocess
import shutil
from pathlib import Path
import urllib.request
import zipfile
import subprocess
import platform

print("Welcome to GTK Rust Template")
while True:
    name = input("Name (e.g. My Awesome App): ")
    if len(name) > 0:
        break
while True:
    project_name = input("Project Name (e.g. my-awesome-app): ")
    if len(project_name) > 0:
        break

while True:
    app_id = input(
        "Application ID (e.g. org.domain.MyAwesomeApp, see: https://developer.gnome.org/documentation/tutorials/application-id.html ): "
    )
    if len(app_id) > 0:
        break


for segment in app_id.split(".")[:-1]:
    if "-" in segment:
        exit("App Id might only contain '-' in the last segment")

author = input("Author: ")
update_contact = input("Email: ")

project_name_alt = project_name.replace("-", "_")

app_path = "/" + "/".join(app_id.split(".")) + "/"
current_dir = Path(os.getcwd())
project_dir = current_dir / project_name

files_url = "https://github.com/Relm4/relm4-template/archive/refs/tags/v0.1.0.zip"
zip_path = current_dir / "relm4-template.zip"
template_path = current_dir / "relm4-template-0.1.0"
online = len(sys.argv) >= 2 and sys.argv[1] == "--online"

CURRENT_APP_ID = "com.belmoussaoui.GtkRustTemplate"
CURRENT_PROJECT_NAME = "gtk-rust-template"
CURRENT_PROJECT_NAME_ALT = "gtk_rust_template"
CURRENT_NAME = "GTK Rust Template"
CURRENT_AUTHOR = "Bilal Elmoussaoui"
CURRENT_EMAIL = "bil.elmoussaoui@gmail.com"
CURRENT_APP_PATH = "/com/belmoussaoui/GtkRustTemplate/"


if project_dir.is_dir():
    wanna_remove = ""
    while wanna_remove not in ["y", "n"]:
        wanna_remove = input(
            "Project already exists, do you want to remove it? [y/n] "
        ).lower()

    if wanna_remove == "y":
        shutil.rmtree(project_dir)
    else:
        exit()

if online:
    zip_destination, _ = urllib.request.urlretrieve(files_url, current_dir / "relm4-template.zip")

    with zipfile.ZipFile(zip_destination, 'r') as zip_ref:
        zip_ref.extractall(current_dir)

    os.remove(zip_destination)

    for item in os.listdir(template_path):
        item_path = os.path.join(template_path, item)
        if "create-project.py" not in item_path:
            remove_path = os.path.join(current_dir, item)
            shutil.move(item_path, remove_path)

    shutil.rmtree(template_path)

items_to_copy = [
    Path(".github"),
    Path("build-aux"),
    Path("data"),
    Path("hooks"),
    Path("po"),
    Path("src"),
    Path(".editorconfig"),
    Path(".gitignore"),
    Path(".gitlab-ci.yml"),
    Path("Cargo.toml"),
    Path("meson_options.txt"),
    Path("meson.build"),
    Path("README.md"),
]

for item in items_to_copy:
    item_path = Path(item)
    if item_path.is_dir():
        shutil.copytree(item_path, project_dir / item)
    else:
        shutil.copyfile(item_path, project_dir / item)


files_with_content_to_rename = [
    Path("build-aux") / "com.belmoussaoui.GtkRustTemplate.Devel.json",
    Path("data") / "com.belmoussaoui.GtkRustTemplate.desktop.in.in",
    Path("data") / "com.belmoussaoui.GtkRustTemplate.gschema.xml.in",
    Path("data") / "com.belmoussaoui.GtkRustTemplate.metainfo.xml.in.in",
    Path("data") / "resources" / "resources.gresource.xml",
    Path("po") / "POTFILES.in",
    Path("src") / "app.rs",
    Path("src") / "main.rs",
    Path("src") / "setup.rs",
    Path("src") / "modals" / "about.rs",
    Path("Cargo.toml"),
    Path("meson.build"),
    Path("meson_options.txt"),
    Path(".gitlab-ci.yml"),
    Path(".github") / "workflows" / "ci.yml",
]

for file in files_with_content_to_rename:
    current_path = project_dir / file

    with open(current_path, "r") as handle:
        content = handle.read()
        content = content.replace(CURRENT_APP_ID, app_id)
        content = content.replace(CURRENT_APP_PATH, app_path)
        content = content.replace(CURRENT_PROJECT_NAME, project_name)
        content = content.replace(CURRENT_PROJECT_NAME_ALT, project_name_alt)
        content = content.replace(CURRENT_NAME, name)
        content = content.replace(CURRENT_AUTHOR, author)
        content = content.replace(CURRENT_EMAIL, update_contact)

    with open(current_path, "w") as handle:
        handle.write(content)

files_to_rename = [
    Path("build-aux") / "com.belmoussaoui.GtkRustTemplate.Devel.json",
    Path("data") / "icons" / "com.belmoussaoui.GtkRustTemplate-symbolic.svg",
    Path("data") / "icons" / "com.belmoussaoui.GtkRustTemplate.svg",
    Path("data") / "icons" / "com.belmoussaoui.GtkRustTemplate.Devel.svg",
    Path("data") / "com.belmoussaoui.GtkRustTemplate.desktop.in.in",
    Path("data") / "com.belmoussaoui.GtkRustTemplate.gschema.xml.in",
    Path("data") / "com.belmoussaoui.GtkRustTemplate.metainfo.xml.in.in",
]


for file in files_to_rename:
    current_path = project_dir / file
    new_path = project_dir / file.parent / str(file.name).replace(CURRENT_APP_ID, app_id)
    shutil.move(current_path, new_path)


subprocess.call(["git", "init"], cwd=project_dir)
# Add all files and commit them
subprocess.call(["git", "add", "-A"], cwd=project_dir)
subprocess.call(["git", "commit", "-m", "Init with GTK Rust Template"], cwd=project_dir)

items_to_delete = [
    ".flatpak-builder",
    ".github",
    "build-aux",
    "data",
    "hooks",
    "po",
    "src",
    ".editorconfig",
    ".gitignore",
    ".gitlab-ci.yml",
    "Cargo.lock",
    "Cargo.toml",
    "LICENSE.md",
    "README.md",
    "create-project.py",
    "meson.build",
    "meson_options.txt",
]

if online:
    for item in items_to_delete:
        item_path = os.path.join(current_dir, item)
        if "create-project.py" not in item_path and project_name not in item_path:
            remove_path = current_dir / item
            if remove_path.is_dir():
                shutil.rmtree(remove_path)
            else:
                os.remove(remove_path)

current_os = platform.system()

if current_os == "Windows":
    subprocess.Popen(f'explorer "{current_dir}"')
elif current_os == "Darwin":  # macOS
    subprocess.Popen(["open", current_dir])
elif current_os == "Linux":
    subprocess.Popen(["xdg-open", current_dir])
else:
    print("Failed to open project path, unsupported operating system.")
    print("Created project at: " + current_dir)