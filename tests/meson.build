if get_option('tests')

test_env = [
  'G_TEST_SRCDIR=@0@'.format(meson.current_source_dir()),
  'G_TEST_BUILDDIR=@0@'.format(meson.current_build_dir()),
  'G_DEBUG=gc-friendly,fatal-warnings',
  'GSETTINGS_BACKEND=memory',
  'MALLOC_CHECK_=2'
]

test_cflags = [
  '-I@0@/../src'.format(meson.current_source_dir()),
  '-I@0@/../src'.format(meson.current_build_dir()),
  '-DEEK_COMPILATION',
  '-DEEKBOARD_COMPILATION'
]

test_link_args = [
  '-fPIC',
]

c_tests = [
]

foreach name : c_tests

    test_sources = [name + '.c']

    t = executable(
        name,
        test_sources,
        squeekboard_resources,
        link_with: libsqueekboard,
        c_args : test_cflags,
        link_args: test_link_args,
        dependencies: deps,       # from src/meson.build
        include_directories: [
            include_directories('..'),
            include_directories('../eek')
        ]
    )

    test(name, t, env: test_env)

endforeach

# The layout test is in the examples directory
# due to the way Cargo builds executables
# and the need to call it manually.

# This is the list of tested builtin layouts.
# Please keep each block alphabetical!
# Please keep shapes (with _) on the same line,
# variants (with +) on separate lines.
foreach layout : [
    # This is the fallback layout,
    # so stays first to make sure it never goes missing.
    'us', 'us_wide',

    # Block: Languages
    'am', 'am_wide',
    'am+phonetic', 'am+phonetic_wide',
    'ara', 'ara_wide',
    'be', 'be_wide',
    'bg', 'bg_wide',
    'bg+phonetic', 'bg+phonetic_wide',
    'br', 'br_wide',
    'ca', 'ca_wide',
    'ch+fr', 'ch+fr_wide',
    'ch+de', 'ch+de_wide',
    'ch', 'ch_wide',
    'cz', 'cz_wide',
    'cz+qwerty', 'cz+qwerty_wide',
    'de', 'de_wide',
    'dk', 'dk_wide',
    'epo', 'epo_wide',
    'es', 'es_wide',
    'es+cat', 'es+cat_wide',
    'fi', 'fi_wide',
    'fr', 'fr_wide',
    'ge', 'ge_wide',
    'gr', 'gr_wide',
    'gr+polytonic', 'gr+polytonic_wide',
    'fr+bepo', 'fr+bepo_wide',
    'hu', 'hu_wide',
    'il', 'il_wide',
    'ir', 'ir_wide',
    'it', 'it_wide',
    'it+fur', 'it+fur_wide',
    'jp+kana','jp+kana_wide',
    'no', 'no_wide',
    'pt', 'pt_wide',
    'pl', 'pl_wide',
    'ro', 'ro_wide',
    'ru', 'ru_wide',
    'se', 'se_wide',
    'si', 'si_wide',
    'th', 'th_wide',
    'tr', 'tr_wide',
    'tr+f', 'tr+f_wide',
    'ua', 'ua_wide',
    'us+colemak', 'us+colemak_wide',
    'us+dvorak', 'us+dvorak_wide',

    # Terminal keyboards
    'terminal/de', 'terminal/de_wide',
    'terminal/es', 'terminal/es_wide',
    'terminal/fr', 'terminal/fr_wide',
    'terminal/us', 'terminal/us_wide',
    'terminal/us+dvorak', 'terminal/us+dvorak_wide',
    
    # Block: Not languages.
    'emoji/us', 'emoji/us_wide',
    'number/us', 'number/us_wide',
    'pin/us', 'pin/us_wide',
]
    extra = []
    if layout.startswith('emoji/')
        extra += ['allow_missing_return']
    endif
    
    timeout = 30

    test(
        'test_layout_' + layout,
        cargo_script,
        args: ['run'] + cargo_build_flags
            + ['--example', 'test_layout', '--', layout]
            + extra,
        timeout: timeout,
        workdir: meson.build_root(),
    )
endforeach

endif

if get_option('find_orphans')
    test('test_find_orphans',
        cargo_script,
        args: ['run'] + cargo_build_flags
            + ['--example', 'find_orphan_layouts',
                '--', meson.source_root() + '/data/keyboards/'],
        timeout: timeout,
        workdir: meson.build_root(),
    )
endif
