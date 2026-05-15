(comment) @comment
(afx_comment) @comment

; AFX — HTML-like template syntax embedded in afx`...` blocks
(afx_opening_element
  (afx_identifier) @tag)
(afx_closing_element
  (afx_identifier) @tag)
(afx_element_self_closing
  (afx_identifier) @tag)

(afx_attribute
  (afx_property_identifier) @tag.attribute)

(afx_text) @string

; EEL — expression language in ${...} blocks
(eel_object_path
  (eel_path_identifier) @variable.special
  (#match? @variable.special "^(this|props)$"))

(eel_object_path
  (eel_path_identifier) @variable)

(eel_object_pair
  key: (eel_property_name) @property)

(eel_method_name) @function

(eel_parameter) @variable

; Fusion DSL
(path_part) @property
(meta_property) @attribute

(prototype_signature
  "prototype" @keyword)

(include_statement
  "include" @keyword.import
  (source_file) @string.special)

(namespace_declaration
  "namespace" @keyword
  (alias_namespace) @namespace)

(type
  name: (type_name) @type)

; AFX punctuation
(afx_opening_element ["<" ">"] @punctuation.bracket)
(afx_closing_element ["<" ">" "/"] @punctuation.bracket)
(afx_element_self_closing ["<" "/>"] @punctuation.bracket)

[
  (package_name)
  (alias_namespace)
] @namespace

(namespace_declaration "=" @operator)
(assignment "=" @operator)
(copy "<" @operator)
(deletion) @operator
(eel_binary_expression operator: _ @operator)
(eel_not_expression ["!" "not"] @operator)

(string) @string
(number) @number
(boolean) @boolean
(null) @constant.builtin

(value_expression
  start: _ @punctuation.delimiter
  end: _ @punctuation.delimiter)

["(" ")" "{" "}" "[" "]"] @punctuation.bracket

[":" "." "?"] @punctuation.delimiter
