;; Auto-indentation: Zed documents @indent, @start and @end captures.
;; Indent one level inside dictionaries and lists.
(dict
    "{" @start
    "}"? @end) @indent

(list
    "(" @start
    ")"? @end) @indent
