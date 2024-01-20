# example `pluginConfigDetail.args`

```nix
# access args in viml
{
  language = "vim";
  code = ''
    s:args['foo'] " bar
  '';
  args = {
    foo = "bar";
  };
}

# access args in lua
{
  language = "lua";
  code = ''
    args.foo -- bar
  '';
  args = {
    foo = "bar"
  };
}
```
