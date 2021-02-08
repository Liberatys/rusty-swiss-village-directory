# frozen_string_literal: true

ENV["BUNDLE_GEMFILE"] = File.expand_path("../Gemfile", File.dirname(__FILE__))
$LOAD_PATH.unshift File.expand_path("../lib", File.dirname(__FILE__))
require "rusty_swiss_village_directory"

require "minitest/autorun"

class SearchTest < Minitest::Test
  def test_that_will_be_skipped; end
end
