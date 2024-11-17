# :gear: Universal Project Manager

Upman is a CLI tool that provides OS and language independent commands for all aspects of project management.

![Crates.io Total Downloads](https://img.shields.io/crates/d/upman?logo=rust&label=Downloads&labelColor=blue)

![Screenshot 2024-11-17 173152](https://github.com/user-attachments/assets/37b8c420-a04a-4ab0-82e3-3809ea0e16f5)

![Windows OS Compatible](https://img.shields.io/badge/Windows-0078D6?style=flat&logo=windows&logoColor=white)
![Mac OS Compatible](https://img.shields.io/badge/macOS-000000?style=flat&logo=apple&logoColor=white)
![Linux OS Compatible](https://img.shields.io/badge/Linux-FCC624?style=flat&logo=linux&logoColor=black)

[View on Crates.io](https://crates.io/crates/upman)

## 🛠️ Installation

### Method #1 (Manual Installation)

Install the binary from Releases for your OS, and place it in a folder recognized by your PATH or add it's location to your PATH.

### Method #2 (Automatic Installation - Crates.io - requires Rust)

Open a terminal (with Rust installed on your machine)

```
cargo install upman
```

### Method #3 (Build from the repo - requires Rust)

Open a terminal (with Rust installed on your machine)

```
git clone https://github.com/james-kaddissi/universal_project_manager.git
cd universal_project_manager
cargo build
```

<br>
<br>
<br>

## :page_facing_up: Command Guide

### Project Creation

```
upman new <PROJECT_NAME> <PROJECT_LANGUAGE> *FLAGS*
```

##### <PROJECT_NAME> - The desired name of the project/root directory.

##### <PROJECT_LANGUAGE> - The main programming language of the project.

##### *FLAGS* - add any amount of these flags to include in project creation. (Flags can be set to true by default in your config file)

- **--git** - initializes the directory as a git repository
- **--ignore** - initializes a .gitignore in the root directory
- **--venv** - initializes a virtual environment in the root directory
- **--license** - initializes your default license (set in your config file) in the root directory
- **--readme** - initializes a README.md in your root directory
- **--tests** - initializes a tests directory in your root directory
- **--docs** - initializes a docs directory in your root directory
- **--docker** - initializes docker in the project

#### Project creation from an existing directory

```
upman init
```

---

<br>

### Package Management

_Only works with languages that have a package manager (pip, cargo, npm, etc.)_

```
upman add <PACKAGE_NAME>
```

##### <PACKAGE_NAME> - the desired name of the package to be added.

---

<br>

### Project Execution

```
upm run
```

##### Runs the main entry point of the project

---

<br>

### Preferences Configuration

```
upm config <MODIFIER> <ARGUMENT>
```

##### <MODIFIER> - defines what configuration you'd like to change

##### <ARGUMENT> - specifies the specific value to change or change too

**Modifiers:**

- _main_ - sets the main path of the project. Argument is the path.
- _defaults_ - toggles the specified (argument) default flag to true or false. Arguments: (git, ignore, docs, etc.)
- _editor_ - sets your default code editor. Argument is the editor name.
- _license_ - sets your default license. Argument is the license name.
- _warnings_ - toggle on or off various warnings. Argument is the warning name (shown in the warning message itself)

---

<br>

### Project Management

```
upm open <PROJECT>
```

##### Opens the specified PROJECT in the terminal and default editor

```
upm delete <PROJECT>
```

##### Deletes the specified PROJECT from your projects list

```
upm list <ARGUMENT>
```

##### Lists specifics about the entered ARGUMENT (editors, templates, licenses, projects, preferences, languages)

---

<br>

### Miscellaneous Tools

```
upm template <ACTION> <TEMPLATE_NAME> **PROJECT_NAME** **PROJECT_LANGUAGE** **PROJECT_MAIN**
```

###### <ACTION> - which template feature to use (save, create, delete)

###### <TEMPLATE_NAME> - name of the template you are modifying

**PROJECT_NAME** **PROJECT_LANGUAGE** **PROJECT_MAIN** - Specifies the details of project creation (only required with the create ACTION)

- **Save** - saves the current directory as a template under the specified name
- **Create** - creates a UPM project using the specified template
- **Delete** - deletes the specified template from your saved templates

<br>

```
upm script <ACTION> <SCRIPT_NAME> **SCRIPT_PATH**
```

##### <ACTION> - which script feature to use (save, delete, add)

##### <SCRIPT_NAME> - name of the script you are modifying

**SCRIPT_PATH** - Specifies the path of the script to save (only required with the save ACTION)

- **Save** - saves the script at the specified path under the specified name
- **Delete** - deletes the specified script from your saved scripts
- **Add** - adds the specified script to your current working directory

<br>

```
upm secrets <ACTION> <SECRET_NAME> <SECRET_VALUE>
```

##### <ACTION> - which secrets feature to use (save/add, delete/remove, show)

##### <SECRET_NAME> - the name of the secret to modify

##### <SECRET_VALUE> the value of the specified secret

- **Save/Add** - Adds a new secret with NAME and VALUE to your .env
- **Delete/Remove** - Removes the specified secret from your .env
- **Show** - Lists the secrets in your .env
