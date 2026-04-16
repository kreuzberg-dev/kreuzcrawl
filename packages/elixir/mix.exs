defmodule Kreuzcrawl.MixProject do
  use Mix.Project

  def project do
    [
      app: :kreuzcrawl,
      version: "0.1.0-rc.2",
      elixir: "~> 1.14",
      compilers: [:rustler] ++ Mix.compilers(),
      description: "High-performance web crawling engine",
      package: package(),
      deps: deps()
    ]
  end

  defp package do
    [
      licenses: ["Elastic-2.0"],
      links: %{"GitHub" => "https://github.com/kreuzberg-dev/kreuzcrawl"}
    ]
  end

  defp deps do
    [
      {:rustler, "~> 0.34"},
      {:credo, "~> 1.7", only: [:dev, :test], runtime: false},
      {:ex_doc, "~> 0.40", only: :dev, runtime: false}
    ]
  end
end
