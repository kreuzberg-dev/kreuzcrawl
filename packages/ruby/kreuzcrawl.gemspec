# frozen_string_literal: true

Gem::Specification.new do |spec|
  spec.name = "kreuzcrawl"
  spec.version = "0.1.0"
  spec.authors = ["Kreuzberg Team"]
  spec.email = ["team@kreuzberg.dev"]

  spec.summary = "High-performance web crawling engine"
  spec.description = "Ruby bindings for kreuzcrawl - a Rust-powered web crawling engine"
  spec.homepage = "https://github.com/kreuzberg-dev/kreuzcrawl"
  spec.license = "Elastic-2.0"
  spec.required_ruby_version = ">= 3.2.0"

  spec.files = Dir["lib/**/*.rb", "ext/**/*.{rs,toml,rb,lock}"]
  spec.require_paths = ["lib"]
  spec.extensions = ["ext/kreuzcrawl_rb/extconf.rb"]

  spec.add_dependency "rb_sys", "~> 0.9"
end
