import os
import subprocess
import shutil


ABS_PATH = os.path.dirname(os.path.realpath(__file__))

print("Welcome to GTK Rust Template")
name = input("Name: ")
project_name = input("Project Name : ")
app_id = input(
    "Application ID (see: https://developer.gnome.org/ChooseApplicationID/): ")
author = input("Author: ")
update_contact = input("Email: ")
project_repo = input("Github/Gitlab repository: ")


REPO_URL = "https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template.git"
CURRENT_APP_ID = "com.belmoussaoui.GtkRustTemplate"
CURRENT_PROJECT_NAME = "gtk-rust-template"
CURRENT_NAME = "GTK Rust Template"
CURRENT_AUTHOR = "Bilal Elmoussaoui"
CURRENT_EMAIL = "bil.elmoussaoui@gmail.com"
CURRENT_APP_PATH = "/com/belmoussaoui/GtkRustTemplate/"
CURRENT_PROJECT_REPO = "https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template"

FILES = [
    "build-aux/com.belmoussaoui.GtkRustTemplateDevel.json",
    "data/icons/com.belmoussaoui.GtkRustTemplate-symbolic.svg",
    "data/icons/com.belmoussaoui.GtkRustTemplate.svg",
    "data/icons/com.belmoussaoui.GtkRustTemplateDevel.svg",
    "data/com.belmoussaoui.GtkRustTemplate.desktop.in.in",
    "data/com.belmoussaoui.GtkRustTemplate.gschema.xml.in",
    "data/com.belmoussaoui.GtkRustTemplate.metainfo.xml.in.in",
    "data/resources.gresource.xml",
    "data/resources/ui/about_dialog.ui.in",
    "data/resources/ui/window.ui.in",
    "po/POTFILES.in",
    "src/application.rs",
    "src/main.rs",
    "src/window.rs",
    "Cargo.toml",
    "meson.build",
    "meson_options.txt",
    ".gitlab-ci.yml"
]

TO_RENAME = [
    "build-aux/com.belmoussaoui.GtkRustTemplateDevel.json",
    "data/icons/com.belmoussaoui.GtkRustTemplate-symbolic.svg",
    "data/icons/com.belmoussaoui.GtkRustTemplate.svg",
    "data/icons/com.belmoussaoui.GtkRustTemplateDevel.svg",
    "data/com.belmoussaoui.GtkRustTemplate.desktop.in.in",
    "data/com.belmoussaoui.GtkRustTemplate.gschema.xml.in",
    "data/com.belmoussaoui.GtkRustTemplate.metainfo.xml.in.in",
]

app_path = "/" + "/".join(app_id.split(".")) + "/"
project_dir = os.path.join(ABS_PATH, project_name)

if os.path.isdir(project_dir):
    wanna_remove = ""
    while wanna_remove.lower() not in ["y", "n"]:
        wanna_remove = input(
            "Project already exists, do you want to remove it? [Y/n]")

    if wanna_remove == "y":
        shutil.rmtree(project_dir)
    else:
        exit()

subprocess.call(['git', 'clone', '--depth', '1',
                 REPO_URL, project_name], cwd=ABS_PATH)

for file in FILES:
    current_path = os.path.join(project_dir, file)

    with open(current_path, 'r') as handle:
        content = handle.read()
        content = content.replace(CURRENT_APP_ID, app_id)
        content = content.replace(CURRENT_APP_PATH, app_path)
        content = content.replace(CURRENT_PROJECT_REPO, project_repo)
        content = content.replace(CURRENT_PROJECT_NAME, project_name)
        content = content.replace(CURRENT_NAME, name)
        content = content.replace(CURRENT_AUTHOR, author)

    with open(current_path, 'w') as handle:
        handle.write(content)

for file in TO_RENAME:
    current_path = os.path.join(project_dir, file)
    new_path = os.path.join(
        project_dir, file.replace(CURRENT_APP_ID, app_id))
    shutil.move(current_path, new_path)


subprocess.call(['git', 'checkout', '--orphan', 'newBranch'], cwd=project_dir)
# Add all files and commit them
subprocess.call(['git', 'add', '-A'], cwd=project_dir)
subprocess.call(
    ['git', 'commit', '-m',  '"Init with GTK Rust Template"'], cwd=project_dir)
# Deletes the master branch
subprocess.call(['git', 'branch', '-D', 'master'], cwd=project_dir)
# Rename the current branch to master
subprocess.call(['git', 'branch', '-m', 'master'], cwd=project_dir)

subprocess.call(['git', 'remote', 'set-url', 'origin',
                 f'{project_repo}.git'], cwd=project_dir)
