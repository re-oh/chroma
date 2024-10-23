# Chroma

Chroma is a text editor / ide, made with simplicity and modularity in mind.

Written in rust using iced.rs, its fast lightweight and easy to modify.

## Plugin System

Chroma offers a rich plugin system based on lua. Chroma can be considered
as just a framework that interprets lua scripts to provide with a full featured
editor. If you want to you can deck your ide out with all the features you want
or just a few simple features. The choice is yours.

Chroma has a set of "Enviorment Variables" that define specific modules of the editor.
So that other modules dont have to worry about finding resources of other modules.

Dont like the defualt file explorer? Write your own, switch it out with the default one.
And you will see that all other modules that depend on it will automatically use your new file explorer.

## Chroma Core

Chroma core is a set of a few basic modules that define a "default" editor, they come preinstalled
but are not required to be used, write your own modules and switch them out with the default ones.
Or go on the offical package manager and download some new ones. The choice is yours.

## Package Manager

All chroma modules when published are built using the chroma package manager. They output 2 files.
a .toml file that contains config items for the module, and a .lua file that contains the actual module.

All modules are stored in a central repository under a MIT license. You can download them and modify them
to your hearts content.

Chroma itself is under a non-comercial lisence, so you can modify it to your hearts content, but you cant sell it.

## Roadmap

By the end of this year (2024) i hope to have the first beta release of chroma out.
This will not include the module system, but a basic text editor with a few features.

The module system is something i plan to work on concrtely after the first beta release. Since
without understanding the core of the editor it would be hard to implement a module system that
istelf will be defining the core of the editor.

## Grapical Designed

Always loved well made high contrast themes. The Chroma theme is a high contrast theme
with vibrant and fun colors that make the editor a joy to use.

Chroma features a custom icon set that is designed to fit the default editor theme & design.

## Conclusion

Chroma is first and foremost made for me. I wanted a text editor that was simple, fast and easy to modify.
I hope that you will find it as useful as i do. If you have any questions or suggestions feel free to reach out to me.

