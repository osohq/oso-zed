(resource_block (resource_type) @name name: (namespaced_identifier) @context) @item
(declaration ["permissions" "roles" "relations"] @name) @item
(test_block header: (test_header name: (string)) @name) @item
(test_setup "setup" @name) @item
(fact_declaration name: (namespaced_identifier) @name) @item
(assertion keyword: ["assert" "assert_not"] @name predicate: (_) @context) @item
(rule_block (rule_functor) @name) @item
