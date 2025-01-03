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
Versatile, adaptable, and fast static site generator written in Rust.
</div>
<br/>

Adduce is a static site generator with the flexibility to bring your web projects to life. Its functionality has two distinct parts: individual and feed.

### Individual
As the name suggests, the individiual functionality is used to generate single pages. It leverages a configuration file per page to grant absolute control over included content and its order.

### Feed
The feed functionality, in contrast to the individual functionality, uses a single configuration file for all documents. Documents are written in markdown. The feed functionality also supports generating an Atom feed.

## Sites Using Adduce
- **Adduce** - [Website](https://adduce.vale.rocks) | [Source Code](https://github.com/DeclanChidlow/Adduce-Site)
- **Mutant Remix** - [Website](https://mutant.revolt.chat) | [Source Code](https://github.com/mutant-remix/website)
- **ToastXC's Site** - [Website](https://toastxc.xyz) | [Source Code](https://github.com/toastxc/toastxc.xyz)
- **Emily Drage's Portfolio** - [Website](https://emilydrage.com) | [Source Code](https://github.com/emdragee/emily.drage)

## Installation

<details>
<summary>Cargo</summary>

```
cargo install adduce
```

</details>

We are in the process of supporting more package managers.

## Building

1. Ensure you have Rust installed.

2. Clone the repository:
```
git clone https://github.com/DeclanChidlow/Adduce
```
3. Enter the repository's directory:
```
cd Adduce
```
4. Build the program:
```
cargo build
```

## Documentation 
Adduce's usage is [documented in the wiki](https://github.com/DeclanChidlow/Adduce/wiki). If you need documentation for the code itself, then you are best off checking the extensive code comments.
