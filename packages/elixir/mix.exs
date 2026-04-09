defmodule KreuzCrawl.MixProject do
  use Mix.Project

  @version "0.1.0"

  def project do
    [
      app: :kreuzcrawl,
      version: @version,
      elixir: "~> 1.14",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      description: "High-performance web crawling engine - Elixir bindings",
      package: package()
    ]
  end

  def application do
    [extra_applications: [:logger]]
  end

  defp deps do
    [
      {:rustler, "~> 0.36.0"},
      {:ex_doc, "~> 0.34", only: :dev, runtime: false}
    ]
  end

  defp package do
    [
      name: "kreuzcrawl",
      licenses: ["Elastic-2.0"],
      links: %{
        "GitHub" => "https://github.com/kreuzberg-dev/kreuzcrawl"
      },
      files: ~w(lib native .formatter.exs mix.exs README.md LICENSE)
    ]
  end
end
