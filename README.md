# jencli

Some weird CLI tool for Jenkins.

## Why?

We have Jenkins `1.204.x` instance with 150+ jobs, `jcli` isn't compatible, `jenkins-cli.jar` doesn't work :D.
The most frictionless way is to use terminal.

## How to use

Put credentials into `config.yml` file and use commands. Script `jencli.sh` provides an example of usage 
with [fzf](https://github.com/junegunn/fzf).

### 1. List jenkins jobs

```shell
jencli list [--mask]
```

**Exclude something from output**

For safety reason you might want to exclude something from `list` command output. Just write it down into config:

```yaml
list:
  exclude:
    - 'something-to-exclude'
```


### 2. Build job by name

```shell
jencli build --name <job-name>
```

## Cache

List command supports cache. To reset cache remove `cache` directory or content.