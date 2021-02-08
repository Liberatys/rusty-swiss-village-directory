# frozen_string_literal: true

require "rusty_swiss_village_directory/version"
require "rutie"

# RustySwissVillageDirectory module behaves as a singleton object with the alternative method
class SwissVillageDirectory
  Rutie.new(:rusty_swiss_village_directory).init "Init_rusty_swiss_village_directory", __dir__
end
