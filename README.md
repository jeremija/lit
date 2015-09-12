# lit

Utility for setting backlight via the `/sys/class/backlight` interface written
in Rust.

# why

This began as an experiment - I wanted to rewrite my
[bright](https://github.com/jeremija/bright) utility in Rust language.

I believe there is still room for improvement, so if you have a suggestion,
or a bug report, feel free to submit a patch or let me know.

# usage

```bash
# increase brightness by 5%
lit +5

# decrease brightness by 5%
lit -5

# set brightness to 70%
lit 70
```

# license

MIT
