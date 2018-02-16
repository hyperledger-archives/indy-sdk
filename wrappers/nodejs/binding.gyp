{
  "targets": [
    {
      "target_name": "indy",
      "include_dirs": [
        "<!(node -e \"require('nan')\")",
        "<(module_root_dir)/../../libindy/include",
      ],
      "sources": [
        "src/indy.cc"
      ],
      "link_settings": {
        "libraries": [
          "<(module_root_dir)/libindy.so"
        ]
      }
    }
  ]
}
