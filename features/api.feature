Feature: API Requests Handling
  In order to ensure the API requests handling works correctly
  As a user of the API
  I want to be able to check the system status, server time, pair info, and Balance

  Scenario: Request system status successfully
    Given Connection to public server API is succesful
    When I request the system status
    Then System status should be online

  Scenario: Request server time successfully
    Given Connection to public server API is succesful
    When I request the server time
    Then Server time difference from system time should be less than 5 seconds

  Scenario: Request XBTUSD pair info successfully
    Given Connection to public server API for ticker is succesful
    When I request the XBTUSD pair
    Then Pair info should not be empty

  Scenario: Request banalnce via private API
    Given Connection to private server API is succesful
    # replace <api_key> and <private_api_key> with the actual values
    When I request the Balance for <API_KEY> and <PRIVATE_API_KEY>
    Then Cash balance should be greater than 0
    And Asset balance should be greater than 0