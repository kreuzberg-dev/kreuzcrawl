%{
  configs: [
    %{
      name: "default",
      files: %{
        included: ["lib/", "test/"],
        excluded: [~r"/_build/", ~r"/deps/"]
      },
      strict: false,
      color: true,
      checks: [
        # Generated structs mirror Rust core types and may have many fields
        {Credo.Check.Warning.StructFieldAmount, false}
      ]
    }
  ]
}
