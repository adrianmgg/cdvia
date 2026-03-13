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

