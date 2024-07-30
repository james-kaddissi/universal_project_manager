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
*new
--git - Initializes the project as a git repository.
--ignore - Requires the --git flag, initializes an ignore

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
~James Kaddissi
