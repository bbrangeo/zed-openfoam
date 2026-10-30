;; Comments
(comment) @comment

;; Generic Key-value pairs and dictionary keywords
(key_value
    keyword: (identifier) @function
)
(dict
    key: (identifier) @type
)

;; Macros
(macro
    "$" @keyword.conditional
    (prev_scope)* @keyword.conditional
    (identifier)* @module
)


;; Directives
"#" @keyword.conditional
(preproc_call
    directive: (identifier)* @keyword.conditional
    argument: (identifier)* @module
)
(
    (preproc_call
        argument: (identifier)* @module
    ) @keyword.conditional
    (#match? @keyword.conditional "ifeq")
)

(
    (preproc_call) @keyword.conditional
    (#match? @keyword.conditional "(else|endif)")
)

;; Literals

(number_literal) @number
(string_literal) @string
(escape_sequence) @string.escape
(boolean) @boolean

;; Treat [m^2 s^-2] the same as if it was put in numbers format
(dimensions dimension: (identifier) @number)

;; Punctuation
[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "#{"
  "#}"
  ";"
] @punctuation

;; Special identifiers

((identifier) @attribute
  (#match? @attribute "^(uniform|non-uniform|and|or)$"))
