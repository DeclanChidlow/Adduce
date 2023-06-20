<h1 align="center">
  Adduce
  
  [![Stars](https://img.shields.io/github/stars/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/stargazers)
  [![Forks](https://img.shields.io/github/forks/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/network/members)
  [![Pull Requests](https://img.shields.io/github/issues-pr/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/pulls)
  [![Issues](https://img.shields.io/github/issues/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/issues)
  [![Contributors](https://img.shields.io/github/contributors/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/graphs/contributors)
  [![Licence](https://img.shields.io/github/license/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/blob/main/LICENCE)
</h1>


## About
Adduce is a static site generator, blog creator and markdown html tool all in one, originally designed to make the singlepage website creation easier it has expanded functionality to support much more

### Adduce lib
Adducelib contains the markdown and HTML engine and will support more schemas in the future

### Adduce Standard
Adduce Standard is the static site generator that powers [Toast's Site](https://toastxc.xyz) and a few others, it is designed to be used as a bash utility with one TOML file per page and absolute control over page content and directory structure.

### Adduce Feed
Adduce Feeds was originally made as a managed tool for Adduce but was found to be more suitable for blogging, unlike Adduce Standard each page or 'blog' uses the same toml file, with a simple wizard for creating it. Feed gives less control over filesystem access for the sake of ease-of-use. Feed is the only service needed for creating a blog outside of hosting.


## Installation
### Manual Install
```bash
git clone https://github.com/DeclanChidlow/Adduce
cd Adduce
sh install.sh
```
### Package Managers
Adduce is currently not avaliable in any package managers but will likely be added to the Arch User Repository soon.

## Dependencies
`wget`

## Documentation 
The project's usage is documented in the wiki.
- [Adduce Feed](https://github.com/DeclanChidlow/Adduce/wiki/Adduce-Feed)
- [Adduce Standard](https://github.com/DeclanChidlow/Adduce/wiki)

If you need documentation on the code itself then you are best off checking the comments.

## Troubleshooting
- Ensure that all dependencies are installed and accessible
- Ensure you have permissions for the working directory

![LGPLv3 Badge](/README_RESOURCES/LGPLv3%20Logo.svg)
