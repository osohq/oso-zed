(resource_block
  (resource_type) @name
  name: (namespaced_identifier) @context) @item

(resource_block
  "global" @name) @item

(declaration
  key: (namespaced_identifier) @name) @item

(declare_statement
  "declare" @name
  name: (namespaced_identifier) @context) @item

(rule_block
  (rule_functor) @name) @item

(test_block
  header: (test_header
    name: (string)) @name) @item

(test_setup
  "setup" @name) @item

(test_fixture
  kind: "fixture" @name
  name: (namespaced_identifier) @context) @item

(fact_declaration
  name: (namespaced_identifier) @name) @item

(assertion
  keyword: [
    "assert"
    "assert_not"
  ] @name
  head: (rule_functor) @context) @item
