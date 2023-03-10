<h1 align="center">
  Adduce
  
  [![Stars](https://img.shields.io/github/stars/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/stargazers)
  [![Forks](https://img.shields.io/github/forks/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/network/members)
  [![Pull Requests](https://img.shields.io/github/issues-pr/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/pulls)
  [![Issues](https://img.shields.io/github/issues/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/issues)
  [![Contributors](https://img.shields.io/github/contributors/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/graphs/contributors)
  [![Licence](https://img.shields.io/github/license/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/blob/main/LICENCE)
</h1>


# About
Adduce is a static site generator, blog creator and markdown html tool all in one, originally designed to make the singlepage website creation easier it has expanded functionality to support much more

### Adduce lib
Adducelib contains the markdown and HTML engine and will support more schemas in the future


### Adduce Standard
Adduce Standard is the SSG that powers [Toast's Site](https://toastxc.xyz) and a few others, it was designed to be used as a bash utility with one toml file per page with absolute control over page content and directories.

### Adduce Feed
Adduce Feeds was originally made as a managed tool for Adduce but was found to be more suitable for blogging, unlike Adduce Standard each page or 'blog' uses the same toml file, with a simple wizard for creating it. Feed gives less control over filesystem access for the sake of ease-of-use. In fact Feed is the only service needed for creating a blog outside of hosting.


# Install Adduce
```bash
git clone https://github.com/toastxc/Adduce.git -b rss
cd Adduce
sh auto/install.sh
```

# Dependencies
```Neovim```
```wget```

# Documentation 


## [Adduce Feed](https://github.com/toastxc/Adduce/wiki/Adduce-Feed)

## [Adduce Standard](https://github.com/toastxc/Adduce/wiki)


## Troubleshooting

## POSIX
- ensure that both neovim and wget are installed and are accessible
- SELinux may interfere with generation, so check for logs or disable it entirely
- filesystem permissions may also be an issue

## Windows
Adduce has not been tested on Windows outside of basic functionality, if there is an issue you believe to be platform specific create an issue with the `Windows` tag.



![LGPLv3 Badge](/README_RESOURCES/LGPLv3%20Logo.svg)
