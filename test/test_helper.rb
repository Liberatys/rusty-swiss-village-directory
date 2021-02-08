$LOAD_PATH.unshift File.expand_path("../../lib", __FILE__)
require "rusty_swiss_village_directory"

require "minitest/autorun"
require 'color_pound_spec_reporter'
Minitest::Reporters.use! [ColorPoundSpecReporter.new]   
