<div align="center">
<h1>
  Adduce
  
  [![Stars](https://img.shields.io/github/stars/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/stargazers)
  [![Forks](https://img.shields.io/github/forks/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/network/members)
  [![Pull Requests](https://img.shields.io/github/issues-pr/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/pulls)
  [![Issues](https://img.shields.io/github/issues/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/issues)
  [![Contributors](https://img.shields.io/github/contributors/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/graphs/contributors)
  [![Licence](https://img.shields.io/github/license/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/blob/main/LICENCE)
</h1>
Versatile static site generator written in Rust.
</div>
<br/>

Adduce is split into two distinct parts. Standard, and Feed.

### Adduce Standard
Adduce Standard is used to generate individual pages. It's designed to be scripted in BASH and leverages a TOML file per page to grant absolute control over content and directory structure.

### Adduce Feed
Originally conceived as a management tool for Adduce, Adduce Feed found its niche in the realm of blogging. In contrast to Adduce Standard, all pages are generated from a single TOML file, and configuration can be simplified through an integrated setup wizard. Feed does sacrifice some filesystem control for user-friendliness and works well when used alongside Adduce Standard.

### Sites Using Adduce
- [Adduce - https://adduce.vale.rocks](https://adduce.vale.rocks)
- [Vale.Rocks - https://vale.rocks](https://vale.rocks)
- [Mutant Remix - https://mutant.revolt.chat](https://mutant.revolt.chat)
- [ToastXC's Site - https://toastxc.xyz](https://toastxc.xyz)

## Installation
### Manual Install
```console
$ git clone https://github.com/DeclanChidlow/Adduce
$ cd Adduce
$ sh install.sh
```

## Dependencies
- `wget`
- `cargo` is required to build.

## Documentation 
The project's usage is [documented in the wiki](https://github.com/DeclanChidlow/Adduce/wiki). If you want documentation for the code itself then you are best off checking the comments.