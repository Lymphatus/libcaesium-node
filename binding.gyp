{
  "targets": [
    {
      "target_name": "libcaesium",
      "sources": [ "libcaesium.cc" ],
      "cflags": ["-Wno-nullability-completeness"],
      "libraries": [
        "-lcaesium", "-L./build/libcaesium/lib/libcaesium.a"
      ]
    }
  ]
}
