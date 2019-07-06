{
  "targets": [
    {
      "target_name": "indynodejs",
      "include_dirs": [
        "<!(node -e \"require('nan')\")",
        "<(module_root_dir)/include",
        "<(module_root_dir)/../../libindy/include"
      ],
      "sources": [
        "src/indy.cc"
      ],
      "link_settings": {
        "conditions": [
          [ 'OS=="win"',
            {
              'library_dirs': [
                "<(module_root_dir)",
                "<!(node -e \"console.log(process.env.LD_LIBRARY_PATH || '')\")"
              ],
              'libraries': [
                "indy.dll.lib"
              ]
            },
            {
              'libraries': [
                "-L<(module_root_dir)",
                "<!(node -e \"console.log((process.env.LD_LIBRARY_PATH || '').split(':').map(a => '-L' + a.trim()).filter(a => a != '-L').join(' '))\")",
                "-lindy"
              ],
            },
          ]
        ],
      }
    }
  ]
}
