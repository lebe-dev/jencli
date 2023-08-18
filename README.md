# jencli

CLI tool for Jenkins.

## Why?

We have Jenkins `1.204.x` instance with 150+ jobs, `jcli` isn't compatible, `jenkins-cli.jar` doesn't work :D.
The most frictionless way is to use terminal.

## Install

```shell
unzip jencli-v0.4.0-linux_x86_64.zip -d /opt/jencli && /opt/jencli
ln -fs /opt/jencli/jencli /usr/bin/jencli 

cp config.yml-dist config.yml
```

Script `jencli.sh` provides an example of usage 
with [fzf](https://github.com/junegunn/fzf) and [jq](https://github.com/jqlang/jq).

## How to use

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

## Performance

List command uses cache. To reset cache remove `cache` directory or content.

## Roadmap

1. Get job console log