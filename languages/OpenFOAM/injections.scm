;; Pass code blocks to Cpp highlighter
(code (code_body) @injection.content
      (#set! injection.language "cpp"))

;; Highlight regex syntax inside literal strings
((string_literal) @injection.content
      (#set! injection.language "regex"))
