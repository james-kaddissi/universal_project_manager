# universal_project_manager

UPM adds CLI commands and functionality to create and manage your programming projects much easier. When working in multiple languages as many coders (especially students) do, it can be tricky to remember all the different nuances in managing projects like packages managers, compilation and execution steps, and dependency handling. UPM adds commands that handle all the differences, and provides a few simple universal commands to manage any programming project in any language.

Plans for the project:
I plan to add a lot of configurability into the project. For example you should be able to configure your own default templates, default packages, default settings and such. I've already implemented some basics of this aspect, in the form of the upmconfig.toml, where you can set the 'git' and 'ignore' to true or false to have them set as flags always used in the 'upm new' command without being specified. More like this will come. Right now I want to lay down the base commands and make sure they work well and bug free across a wide variety of languages. As for languages the project will eventually be compatible with virtually any language, as for now I will add some of the main ones and slowly add more over time.

COMMANDS

$upm new <PROJECT_NAME> <LANGUAGE> - Creates a new project in a new directory named PROJECT_NAME in the specified LANGUAGE.

$upm run - Must be run from the root of a upm project. Runs the specified main entry point of the upm project.

$upm init <LANGUAGE> <MAIN> - Initializes the current directory as a upm project if not already specified as one. Optionally takes LANGUAGE and MAIN, but if not passed it will prompt for them.

$upm add <PACKAGE_NAME> - Package manager. Adds the named package to the current project.

FLAGS

--git - Initializes the project as a git repository.

--ignore - Requires the --git flag, initializes an ignore

--venv - Initializes the project as

--license - Initializes your default license in the project

--readme - Initializes a readme in the project

--tests - Initializes a tests directory in the project

--docs - Initializes a docs directory in the project

--docker - Initializes docker in the project

SUPPORTED LANGUAGES:

Python

C

C++

C#

Rust

Javascript

React/HTML

Java

Ruby

MORE COMING SOON

Everything i have to do:

Lets break down every main command and what it should be in the final version:

a. Additionally warn all non upm project upm init.

b. Additionally always offer prompts if missing parameter.

c. Additionally address the issue of user not having necessary technologies for a step like for example upm run but not having a certain compiler. Need to warn user and offer an easy fix or automatic installation on the first upm run if viable.

d. Additionally adress the path issues using a constant string instead of automatic configuration.

upm new - upm new should take in a language or optional alternate custom structure. For each language or custom structure a configurable template is used and created with the projects name. This command should take flags that can be toggleable on from project to project. Say you want docker in your project add --docker. You can set certain flags to always be true.

upm template new - simply creates a new upm project from an alternate specified template.

upm add - for whatever language you are using, if appropriate (so for C it wouldn't do anything because C doesn't have packages), it should find a package of that name, if it exists then install it into the project, and add it to any dependency logs if standard for that language (requirements.txt etc.).

upm run <OPTIONAL_SCRIPT> - runs the main entry point of the project or the optional script path if provided, using the method of running a script for that language.

*upm script -*

*a. upm script save
<src><src_script> <save_name> - saves the script at path src_script to the name save_name*</save_name></src_script></src>

*b. upm script add <save_name> - adds the script saved under save_name (if it exists) to the current upm project (if in a upm project).*

*upm init language_ main_ - Initializes the current root project as a upm project with the language and main path being passed as paramaters.*

*upm open project_name - changes terminal directory to projects root and opens the root in the configured editor. project_name must be recognized as a upm project.* Finished but needs platform checks for Unix.

upm list <ARGUMENT_> - lists details about the argument:

*a. editors - lists supported text editors (configurable by upm config)*

*b. templates - lists the names of all templates saved by your UPM (default + custom)* May need to remove README?

*c. licenses - lists the names of all licenses saved by your UPM (default + custom)*

*d. projects - lists the names of all projects with their relevant language/main/path info*

*f. preferences - lists the preference configuration of your version.*

*upm delete optional_name - deletes the current directory and removes from project list or deletes the name of the upm project supplied.*

determine a finalize list of supported languages.

These languages are tested working for project creation and running on Windows:

Ruby

HTML

C#

Python

Java

C

C++

Javascript

React

Thes languages need to be added:

SQL

ASM

PHP

R

Go

Swift

Perl

Obj-C

Pascal

Kotlin

TypeScript

Scala

Shell

Dart

Groovy

Haskell

Erlang

COBOL

Fortran

Lisp

Lua

MATLAB
