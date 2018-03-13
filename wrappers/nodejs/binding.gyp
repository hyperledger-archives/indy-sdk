{
  "targets": [
    {
      "target_name": "indy",
      "include_dirs": [
        "<!(node -e \"require('nan')\")",
        "<(module_root_dir)/include",
      ],
      "sources": [
        "src/indy.cc"
      ],
      "link_settings": {
        "libraries": [
          "-lindy"
        ]
      }
    }
  ]
}
