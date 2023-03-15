# discord bot message shim

This repo holds the code for a rust based cli which enables manually sending, editing and deleting messages from a bot

## How to use(looking at you banan)

You need to specify a channel id and a bot token, you should be able to do this via cli options before your action but I prefer to do it via the config file, on linux this directory is `$HOME/.config/DBMS/config.yaml` I think it can be changed via cli option but idk why you'd do that.

### editing a message

the base command is `DBMS edit`, you also need to specify the message id you're editing and what the new message contents are, the id is specified via the `-m` option, note that the message id is the number after the dash if you copy message id from discord and the message contents can either be specified via stdin if you pass the `-s` cli option or as a cli arg via the `-t` arg with the message contents right after.

Example:

```sh
DBMS edit -m 691271049928769587 -s
```

in this example you'd type out the new message and then hit ctrl+d in order to specify the end or you could pipe in an existing file some way or another

### Sending a message

example:

```sh
DBMS send --text "test"
```

there's no support for stdin when using send at this time but copying the code from the place where edit is done should be relatively easy

### Additional commands

you'll probably not use either of these but there are also retrieve and delete commands as well but both only use some of the options that the edit command does and you can run their help commands to figure them out.
