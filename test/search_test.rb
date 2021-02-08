require "test_helper"
require 'stop_watch'

class SearcherTest < Minitest::Test
  def test_it_rust
    p "RUST"
    watch = StopWatch::Timer.new
    watch.mark
    assert_equal SwissVillageDirectory.by_name("Aeugst am Albis").name, "Aeugst am Albis"
    p watch.mark
  end
end
