# cd via
*`cd`* to the same directory you're already in,
but *via* a different symlink.

## usage
```
/
├─ foo
│  └─ a
│     └─ b
└─ bar
   └─ qux
      └─ a -> ../../foo/a
```

```
> pwd
/foo/a
> cd $(cdvia /bar/qux/a)
> pwd
/bar/qux/a/b
> cd $(cdvia /foo)
> pwd
/foo/a/b
```

Normal invocation outputs the new path to stdout.

Can also use `--format bash_eval` for a version that can be used like
```bash
eval $(cdvia /foo/bar --format bash_eval)
```

Also supports formats `sh_eval` and `fish_eval` for those shells,
and `bash_escaped`, `sh_escaped`, and `fish_escaped` to get the escaped version
but without the accompanying `cd ` command.

