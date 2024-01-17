# whale_fall
- a code generator based on data flow style
## name meaning: 
- 一鲸落,万物生(When a whale falls, all things live)

## install
- cargo install whale_fall

## config template,default name is whale.toml
```
[basic]
package = "example"  # project package
root = "../example"  # project root

[[composer]]  # composer array
parsers = ["gm"]  # parser array
processers = ["miman"] # processer array
generators = ["miman"] # generator array
outputers = ["go"] # outputer array

[[composer]]
parsers = ["gm"]
processers = ["miman"]
generators = ["miman"]
outputers = ["go", "md"]
```


## usage
- cd the project root, config the whale.toml,then execute the command `whale_fall generate` 