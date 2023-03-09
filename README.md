<h1 align="center">
  Adduce
  
  [![Stars](https://img.shields.io/github/stars/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/stargazers)
  [![Forks](https://img.shields.io/github/forks/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/network/members)
  [![Pull Requests](https://img.shields.io/github/issues-pr/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/pulls)
  [![Issues](https://img.shields.io/github/issues/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/issues)
  [![Contributors](https://img.shields.io/github/contributors/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/graphs/contributors)
  [![Licence](https://img.shields.io/github/license/DeclanChidlow/Adduce?style=flat-square&logoColor=white)](https://github.com/DeclanChidlow/Adduce/blob/main/LICENCE)
</h1>

# Install Adduce
```bash
git clone https://github.com/toastxc/Adduce.git -b rss # i think that's how branches work? you need the rss branch
cd Adduce
sh auto/install.sh
```

# Addduce feed
Adduce feed is a managed version of Adduce, with the purpose of creating blogs or other simple documents. Most of the same functionality of Adduce is in Adduce feed but not all

## Generate feed conf
cont.toml in this context is the config for all documents, it can contain stylesheets, preloaded html/md and an authors name
This can be created manually, or with a wizard using conf generate
```bash
adduce feed
adduce feed conf generate
```

## Make and manage documents
Each document has a unique name, and can all be managed in feed edit / new
Adduce feed only supports one editable markdown file per document.
```bash
adduce feed new document
adduce feed edit document
```

## Publish document
Currently the only supported schema is HTML
```bash
adduce feed publish document html
firefox --new-tab ./feed/export/document.html
```

## Workflow Example
This is a simple example of how adduce feed could be used
```bash
nohup watch "adduce feed publish document html" &
firefox --new-tab ./feed/export/document.html
adduce feed edit document
```

![LGPLv3 Badge](/README_RESOURCES/LGPLv3%20Logo.svg)
