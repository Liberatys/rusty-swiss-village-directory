# frozen_string_literal: true

require "test_helper"
require "stop_watch"

class SearcherTest < Minitest::Test
  def test_it_rust
    watch = StopWatch::Timer.new
    watch.mark
    1000.times do |_x|
      assert_equal SwissVillageDirectory.by_name("Aeugst am Albis").name, "Aeugst am Albis"
      watch.mark
    end
    watch.mark
    p watch.ha
    p watch.h
  end
end
