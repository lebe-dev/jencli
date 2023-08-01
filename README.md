# jencli

Some weird CLI tool for Jenkins.

## Why?

We have Jenkins `1.204.x` instance with 150+ jobs, `jcli` isn't compatible, `jenkins-cli.jar` doesn't work.

## How to use

Prepare `config.yml` file.

**1. List jenkins jobs**

```shell
jencli list [--mask]
```