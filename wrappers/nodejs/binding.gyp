{
  "targets": [
    {
      "target_name": "indy",
      "include_dirs": [
        "<!(node -e \"require('nan')\")"
      ],
      "sources": [
        "src/indy.cc"
      ]
    }
  ]
}
