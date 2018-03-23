{
  "targets": [
    {
      "target_name": "indynodejs",
      "include_dirs": [
        "<!(node -e \"require('nan')\")",
        "<(module_root_dir)/include",
      ],
      "sources": [
        "src/indy.cc"
      ],
      "link_settings": {
        "libraries": [
          "-L<(module_root_dir)",
          "-lindy"
        ]
      }
    }
  ]
}
