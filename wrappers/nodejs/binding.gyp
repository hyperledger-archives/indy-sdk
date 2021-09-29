{
  "targets": [
    {
      "target_name": "indynodejs",
      "include_dirs": ["<!(node -e \"require('nan')\")", "<(module_root_dir)/include"],
      "sources": ["src/indy.cc"],
      "conditions": [
        [
          "OS=='mac'",
          {
            "library_dirs": ["/usr/local/lib/", "/opt/homebrew/lib/"],
            "link_settings": {
              "libraries": ["libindy.dylib"]
            }
          }
        ],
        [
          "OS=='win'",
          {
            "library_dirs": ["<(module_root_dir)"],
            "libraries": ["indy.dll.lib"]
          }
        ],
        [
          "OS=='linux'",
          {
            "library_dirs": ["/usr/local/lib/", "/usr/lib/"],
            "link_settings": {
              "libraries": ["libindy.so"]
            }
          }
        ]
      ]
    }
  ]
}
