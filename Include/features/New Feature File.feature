#Author: your.email@your.domain.com
#Keywords Summary :
#Feature: List of scenarios.
#Scenario: Business rule through list of steps with arguments.
#Given: Some precondition step
#When: Some key actions
#Then: To observe outcomes or validation
#And,But: To enumerate more Given,When,Then steps
#Scenario Outline: List of steps for data-driven as an Examples and <placeholder>
#Examples: Container for s table
#Background: List of steps run before each of the scenarios
#""" (Doc Strings)
#| (Data Tables)
#@ (Tags/Labels):To group Scenarios
#<> (placeholder)
#""
## (Comments)
#Sample Feature Definition Template
@tag
Feature: Login Functionality
  I want to use this template for my feature file

  @tag1
  Scenario Outline: Positive Scenario of login functionality
    Given visit at url
    When verify landing page
    Then enter the user_name <username>
    And enter the password <password>
    Then click on login button
    And verify the landing page

    Examples: 
      | username      | passowrd     |
      | standard_user | secret_sauce |
