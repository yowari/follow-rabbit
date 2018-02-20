# Follow the Rabbit

Find secrets phrases corresponding to anagrams of `poultry outwits ants` with
definied MD5 hash.

This program solves the Code Challenge provided by [trustpilot](http://followthewhiterabbit.trustpilot.com/cs/step3.html).

# Getting Started

## Concept

The main idea is to construct a trie of letters and check each time if character
can be consumed. If not, then no need to test the next characters.

### Example:

```
<> : last character of a word

            ''
         /      \     \
       /         \     \
     't'         'w'    'y'
      |           |      |
     'r'         'a'    'o'
      |           |      |
     'u'         'n'   <'u'>
    /    \        |
   's'   't'    <'t'>
    |      |      |
  <'t'>  <'h'>  <'s'>
  /   \
 'p'  'i'
  |    |
 'i'  'n'
  |    |
 'l' <'g'>
  |
 'o'
  |
<'t'>
```

## Build

As for now, to build the project you need nightly Rust.

```
cargo build
```

Or with [rustup](https://www.rustup.rs/):

```
rustup run nightly cargo build
```

It is encouraged to use the option `--release` for better performance.

## Visual Studio Code

Tasks are configured to run with rustup. It is also possible to debug the
project thanks to [LLDB Debugger](https://marketplace.visualstudio.com/items?itemName=vadimcn.vscode-lldb).

# Usage

Once the project compiled, you can find the binary either on
`./target/release/follow-rabbit` or `./target/debug/follow-rabbit` depending
on the chosen profile.

```
$ follow-rabbit --help
Follow the Rabbit and see how deep the hole goes. 1.0.0
yowari <yowari@outlook.com>
Search for anagrams

USAGE:
    follow-rabbit [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --dictionary <FILE>    Select the dictionary file. Default = wordlist
    -w, --words <INTEGER>      Set the maximum number of words in an angram. Default = 4
    -l, --length <INTEGER>     Set the minimum length of a word. Default = 2
    -o, --output <FILE>        Output file path. Default = anagrams
```

# Results

Of course I will not give you the secrets phrases. *Find it yourself :stuck_out_tongue:*. But I
can share some benches.

During the benches, the release build was used. RAM consumption was about
**3 Mo**. Thanks to Rust not having VM or garbage collector :sunglasses:

## 1. The easiest secret phrase

```
$ follow-rabbit --words=3 --length=5
```

Execution time was about `3 secs`.

## 2. The more difficult secret phrase

```
$ follow-rabbit --words=3 --length=2
```

Execution time was about `1 minute`.

## 3. The hard secret phrase

```
$ follow-rabbit --words=4 --length=2
```

Execution time was about `1 hour`.
