Feature: A taxi database to start from
  Later episodes start where real reports start: data in a normal database.
  We build data/taxi.db from the public TLC files.

  Scenario: Build the taxi database from the TLC files
    Given a trips Parquet file with 3 trips
    And a zone lookup CSV with 2 zones
    When I build the taxi database
    Then the "trips" table has 3 rows
    And the "zones" table has 2 rows
    And trip 1 was picked up at "2024-01-01 00:57:55"
    And trip 2 has no passenger count

  Scenario: A failed build leaves no half-written database
    Given a zone lookup CSV with 2 zones
    But the trips Parquet file is missing
    When I build the taxi database
    Then the build fails
    And there is no taxi database
