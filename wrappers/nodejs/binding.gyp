# Location
# macOS Intel: /usr/local/lib/libindy.dylib
# macOS Apple: /opt/homebrew/lib/libindy.dylib
# Linux      : /usr/lib/libindy.dylib OR /usr/local/lib/libindy.dylib
# Windows    : anywhere & set LD_LIBRARY_PATH in your environment variables to the /lib folder

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
            "library_dirs": [
              "<(module_root_dir)",
              "<!(node -e \"console.log(process.env.LD_LIBRARY_PATH || '')\")"
            ],
            "libraries": ["indy.dll.lib"]
          }
        ],
      ]
    }
  ]
}
