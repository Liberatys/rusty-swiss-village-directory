# frozen_string_literal: true

lib = File.expand_path("../lib", __FILE__)
$LOAD_PATH.unshift(lib) unless $LOAD_PATH.include?(lib)
require_relative "lib/rusty_swiss_village_directory/version"

Gem::Specification.new do |spec|
  spec.name          = "rusty-swiss-village-directory"
  spec.version       = RustySwissVillageDirectory::VERSION
  spec.authors       = ["Nick Flueckiger"]
  spec.email         = ["nick.flueckiger@renuo.ch"]

  spec.summary       = "ac"
  spec.description   = "a"
  spec.homepage      = "https://github.com/Liberatys/rusty-swiss-village-directory"
  spec.license       = "MIT"
  spec.required_ruby_version = Gem::Requirement.new(">= 2.4.0")

  spec.metadata["allowed_push_host"] = "a"

  spec.metadata["homepage_uri"] = spec.homepage
  spec.metadata["source_code_uri"] = "https://github.com/Liberatys/rusty-swiss-village-directory"
  spec.metadata["changelog_uri"] = "https://github.com/Liberatys/rusty-swiss-village-directory"

  spec.files         = Dir.chdir(File.expand_path('..', __FILE__)) do
    `git ls-files -z`.split("\x0").reject { |f| f.match(%r{^(test|spec|features)/}) }
  end

  spec.require_paths = ["lib"]

  spec.extensions = ["ext/Rakefile"]

  spec.add_dependency "bundler", ">= 1.12"
  spec.add_dependency "rake"
  spec.add_dependency "rutie", "~> 0.0.3"
  spec.add_dependency "thermite", "~> 0.13.0"
  spec.add_development_dependency "minitest"
  spec.add_development_dependency "rubocop", "0.53"
  spec.add_development_dependency "stop_watch", "~> 1.0"
end
