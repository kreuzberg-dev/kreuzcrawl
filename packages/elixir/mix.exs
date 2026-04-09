defmodule Kreuzcrawl.MixProject do
  use Mix.Project

  @version "0.1.0"
  @source_url "https://github.com/kreuzberg-dev/kreuzcrawl"

  def project do
    [
      app: :kreuzcrawl,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      description: "High-performance web crawling engine",
      package: package(),
      docs: docs(),
      source_url: @source_url,
      rustler_crates: [kreuzcrawl: [mode: :release]]
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [
      {:rustler, "~> 0.37.0", optional: true, runtime: false},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.34", only: :dev, runtime: false}
    ]
  end

  defp package do
    [
      licenses: ["Elastic-2.0"],
      links: %{GitHub: @source_url},
      files: ~w(
        lib
        native/kreuzcrawl_nif/src
        native/kreuzcrawl_nif/Cargo.toml
        mix.exs
        README.md
        .formatter.exs
      )
    ]
  end

  defp docs do
    [
      main: "Kreuzcrawl",
      source_url: @source_url,
      extras: ["README.md"]
    ]
  end
end
