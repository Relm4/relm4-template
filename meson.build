project('gtk-rs-template', 
        'rust',
        version: '0.0.1',
        license: 'MIT',
	meson_version : '>= 0.50')

dependency('glib-2.0', version: '>= 2.56')
dependency('gio-2.0', version: '>= 2.56')
dependency('gdk-pixbuf-2.0')
dependency('gtk+-3.0', version: '>= 3.24.7')


version = meson.project_version()
version_array = version.split('.')
major_version = version_array[0].to_int()
minor_version = version_array[1].to_int()
version_micro = version_array[2].to_int()

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')
localedir = prefix / get_option('localedir')

datadir = prefix / get_option('datadir')
pkgdatadir = datadir / meson.project_name()
iconsdir = datadir / 'icons'
podir =meson.source_root () / 'po'
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
cargo_script = find_program('build-aux/cargo.sh')

subdir('src')

meson.add_dist_script(
  'build-aux/dist-vendor.sh',
  meson.build_root() / 'meson-dist' / meson.project_name() + '-' + version,
  meson.source_root()
)

if get_option('profile') == 'development'
    # Setup pre-commit hook for ensuring coding style is always consistent
    message('Setting up git pre-commit hook..')
    run_command('cp', '-f', 'hooks/pre-commit.hook', '.git/hooks/pre-commit')
endif


meson.add_install_script('build-aux/meson_post_install.py')
