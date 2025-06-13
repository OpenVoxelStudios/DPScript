# IR Shortcuts

> `data set: a, b;` \
`data modify storage store!(a) path!(a) set from value b`

> `data copy: a, b;` \
`data modify storage store!(a) path!(a) set from storage store!(b) path!(a)`

> `data append: a, b;` \
`data modify storage store!(a) path!(a) append value b` \
`data modify storage store!(a) path!(a) append from storage store!(b) path!(b)`

> `call: a;` \
`function a`

> `goto: $block0;` \
`function [block0]`

> `execute: '@a', $block0;`
`execute as @a run function [block0]`

> `argument set: 0, 'whatever';`
`data modify storage dpscript:core/args __arg_0 set value "whatever"`

> `argument set: 0, whatever;`
`data modify storage dpscript:core/args __arg_0 set from storage store!(whatever) path!(whatever)`

> `argument get: 0, whatever;`
`data modify storage store!(whatever) path!(whatever) set from storage dpscript:core/args __arg_0`

> `argument clear;`
`data merge storage dpscript:core/args {}`
