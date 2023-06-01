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

**for Mac OS**  

```
brew tap asweed888/homebrew-saba
brew install saba
```

**for Linux**

```
sudo curl -L https://github.com/asweed888/saba/releases/download/{Any version}/saba_linux_x86_64.tar.gz -o - | tar -xzvf - && sudo mv ./saba /bin
```

**other**  

It can be installed from the release page.  
https://github.com/asweed888/saba/releases

## Usage
The use of **saba** is very simple.
All you need to do is create a file called **saba.yml** and describe a simple structure.

## Enable completion

**for Mac OS**

```
saba completion bash > $(brew --prefix)/etc/bash_completion.d/saba
```
