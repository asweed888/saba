# saba

`saba` is a very simple declarative development framework.  
The major version of `saba` has not yet reached v1 and is still in the verification phase.  
Also, it is still unknown how `saba` will solve development problems.  
However, `saba`'s approach to development, declarative development,   
has the potential to make source code more maintainable in that it is implemented while writing specifications.

## feature

- Declarative Development
- Support for multiple dynamically typed languages (currently only python)

## installation
Installation is easy with an installation script.
```
curl -sSL -H 'Cache-Control: no-cache' https://raw.githubusercontent.com/asweed888/saba/main/install.sh | bash
```

The second and subsequent updates can be performed using the alias registered during the first installation.
```
saba_install
```

## Usage
The use of **saba** is very simple.
All you need to do is create a file called **saba.yml** and describe a simple structure.

## Enable completion

**for Mac OS**

```
saba completion bash > $(brew --prefix)/etc/bash_completion.d/saba
```
