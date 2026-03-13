# cd via
*`cd`* to the same directory you're already in,
but *via* a different symlink.

## usage
```
/
├─ foo
│  └─ a -> ../bar/qux/a
└─ bar
   └─ qux
      └─ a
         └─ b
            └─ c
```

```
> pwd
/bar/qux/a/b/c
> cd $(cdvia /foo/a)
> pwd
/foo/a/b/c
> cd $(cdvia /)
> pwd
/bar/qux/a/b/c
```

Normal invocation outputs the new path to stdout.

Can also use `--format bash_eval` for a version that can be used like
```bash
eval $(cdvia /foo/bar --format bash_eval)
```

Also supports formats `sh_eval` and `fish_eval` for those shells,
and `bash_escaped`, `sh_escaped`, and `fish_escaped` to get the escaped version
but without the accompanying `cd ` command.

### sample bash function
bash function to call cdvia easily and without needing to use the eval feature.
```bash
function cdvia { if _cdvia_dest="$(command cdvia --format string -- "$1")"; then cd "$_cdvia_dest"; fi }
```
(note the use of `command` which allows the underlying binary and the function to both be called `cdvia`)  
(note also the use of `--` so that paths starting with hyphens won't have any problems)
