subdir('icons')
# Desktop file
desktop_conf = configuration_data()
desktop_conf.set('icon', application_id)
desktop_file = i18n.merge_file (
  type: 'desktop',
  input: configure_file(
    input: 'com.bilelmoussaoui.GtkRustTemplate.desktop.in.in',
    output: '@BASENAME@',
    configuration: desktop_conf
  ),
  output: '@0@.desktop'.format(application_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'applications'
)
# Validate Desktop file
desktop_file_validate = find_program('desktop-file-validate', required: false)
if desktop_file_validate.found()
  test(
    'validate-desktop',
    desktop_file_validate,
    args: [
      desktop_file.full_path()
    ]
  )
endif

# Appdata
appdata_conf = configuration_data()
appdata_conf.set('app-id', application_id)
appdata_conf.set('gettext-package', gettext_package)
appdata_file = i18n.merge_file (
  input: configure_file(
    input: 'com.bilelmoussaoui.GtkRustTemplate.appdata.xml.in.in',
    output: '@BASENAME@',
    configuration: appdata_conf
  ),
  output: '@0@.appdata.xml'.format(application_id),
  po_dir: podir,
  install: true,
  install_dir: datadir / 'metainfo'
)
# Validate Appdata
appstream_util = find_program('appstream-util', required: false)
if appstream_util.found()
  test(
    'validate-appdata', appstream_util,
    args: [
      'validate-relax', '--nonet', appdata_file.full_path()
    ]
  )
endif

# GSchema
gschema_conf = configuration_data()
gschema_conf.set('app-id', application_id)
gschema_conf.set('gettext-package', gettext_package)
configure_file(
    input: 'com.bilelmoussaoui.GtkRustTemplate.gschema.xml.in',
    output: '@0@.gschema.xml'.format(application_id),
    configuration: gschema_conf,
    install: true,
    install_dir: datadir / 'glib-2.0' / 'schemas'
)

# Resources
ui_config = configuration_data()
ui_config.set('app-id', application_id)
ui_config.set('version', meson.project_version() + version_suffix)
ui_preconfigured_files = files(
  'resources/ui/about_dialog.ui.in',
  'resources/ui/window.ui.in',
)
ui_dependencies = []
foreach ui_file: ui_preconfigured_files
  ui_dependencies += configure_file(
    input:  ui_file,
    output: '@BASENAME@',
    configuration: ui_config
  )
endforeach

resources = gnome.compile_resources(
  'resources',
  'resources.gresource.xml',
  gresource_bundle: true,
  source_dir: meson.current_build_dir(),
  dependencies: ui_dependencies
)

