# frozen_string_literal: true

$LOAD_PATH.unshift File.expand_path("../lib", __dir__)
require "rusty_swiss_village_directory"

require "minitest/autorun"
require "color_pound_spec_reporter"
Minitest::Reporters.use! [ColorPoundSpecReporter.new]
