project('gtk-rs-template', 
        'rust',
        version: '0.1'
        licence: 'MIT')


version = meson.project_version()
version_array = version.split('.')
major_version = version_array[0].to_int()
minor_version = version_array[1].to_int()
version_micro = version_array[2].to_int()

prefix = get_option('prefix')
bindir = join_paths(prefix, get_option('bindir'))
localedir = join_paths(prefix, get_option('localedir'))

datadir = get_option('datadir')
pkgdatadir = join_paths(datadir, meson.project_name())
icondir = join_paths(datadir, 'icons')
podir = join_paths (meson.source_root (), 'po')
gettext_package = meson.project_name()


if get_option('profile') == 'development'
  profile = 'Devel'
  name_suffix = ' (Devel)'
  vcs_tag = run_command('git', 'rev-parse', '--short', 'HEAD').stdout().strip()
  if vcs_tag == ''
    version_suffix = '-devel'
  else
    version_suffix = '-@0@'.format (vcs_tag)
  endif
else
  profile = ''
  name_suffix = ''
  version_suffix = ''
endif

application_id = 'com.bilelmoussaoui.GtkRustTemplate@0@'.format(profile)


i18n = import('i18n')
gnome = import('gnome')

subdir('po')
subdir('data')

cargo = find_program('cargo', required: false)
gresource = find_program('glib-compile-resources', required: true)
cargo_vendor = find_program('cargo-vendor', required: false)
cargo_script = find_program('build-aux/cargo.sh')

subdir('src')

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  join_paths(meson.build_root(), 'meson-dist', meson.project_name() + '-' + shortwave_version),
  meson.source_root()
)

meson.add_install_script('build-aux/meson_post_install.py')
