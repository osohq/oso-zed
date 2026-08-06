; highlights.scm
[
  "and"
  "assert"
  "assert_not"
  "declare"
  "extends"
  "fixture"
  "global"
  "if"
  "iff"
  "in"
  "matches!"
  "matches"
  "not"
  "on"
  "or"
  "setup"
  "test"
  "type"
] @keyword

(rule_flags) @attribute

(current_unix_time) @constant.builtin

(namespaced_identifier) @variable

(string) @string

(boolean) @boolean

(number) @number

(resource_block
  name: (namespaced_identifier) @type.definition)

(resource_type) @constructor

(comparison_operator) @operator

(assignment_operator) @operator

(unify_expression
  "=" @operator)

(fact_declaration
  name: (namespaced_identifier) @function)

(declare_statement
  name: (namespaced_identifier) @function)

(rule_functor
  name: (namespaced_identifier) @function)

(call
  name: (namespaced_identifier) @function.call)

(specializer
  name: (namespaced_identifier) @variable.parameter)

(specializer
  type: (namespaced_identifier) @type)

(matches_expression
  type: (namespaced_identifier) @type)

(dict_field
  value: (namespaced_identifier) @type)

(object_literal
  name: (namespaced_identifier) @type)

(comment) @comment

; test stuff
(test_header
  "test" @module)

(test_header
  name: (string) @label)

(test_setup
  "setup" @module)

(test_fixture
  keyword: "test" @module
  kind: "fixture" @module
  name: (namespaced_identifier) @label)

(fixture_reference
  name: (namespaced_identifier) @label)
