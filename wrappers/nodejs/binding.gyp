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
      "conditions": [
        ["OS=='linux'", {
          "link_settings": {
            "libraries": [
              "<(module_root_dir)/libindy.so"
            ]
          }
        }],
        ["OS=='mac'", {
          "link_settings": {
            "libraries": [
              "<(module_root_dir)/libindy.dylib"
            ]
          }
        }],
        ["OS=='win'", {
          "link_settings": {
            "libraries": [
              "<(module_root_dir)/libindy.dll"
            ]
          }
        }]
      ]
    }
  ]
}
