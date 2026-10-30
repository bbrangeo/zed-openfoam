;; Outline / structure: Zed requires @name (item name) and @item captures.
;; Dictionaries are named items; key-value pairs are anonymous items.
(dict
    key: (identifier) @name) @item

(dict
    key: (string_literal) @name) @item

(key_value
    keyword: (identifier) @name) @item
