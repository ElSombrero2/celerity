# ✨ Celerity.io
Is a simple project manager that include a CLI tool help you to manage your projects and generate them based on a simple template.  
## Why should I use Celerity.io? 🤔
Because it's "Vita Malagasy". 🫡 (Funny Joke that only a few people will understand)
# How to run
Considere that this tool is `only available in dev mode` now  
It will allow you to init a new project from the existing templates,  
and will show all the existing template in the example templates folder  

`Celerity.io` will search your config to .config/configuration.json  
If the file does not exist then run the Github Login command.  
Running this command will initialize all the use configuration.  
For more information refer to the help section or write an issue on this repository.
## CLI
![image](https://res.cloudinary.com/dcsdcjmug/image/upload/v1710710902/lbqkfbztij8iocykkipa.gif)

## Desktop

### Embed Kanban board

![image](https://res.cloudinary.com/dcsdcjmug/image/upload/v1710806987/a1tri0arj5efcqi18puh.gif)

### Figma integration

![image](https://res.cloudinary.com/dcsdcjmug/image/upload/v1710806987/zsu7m9jmjgsveddnqmjy.gif)

## Usage

```bash
Celerity is a simple tool to init your project based on templates

Usage: celerity.exe [OPTIONS] [COMMAND]

Commands:
  init         Initialize your project
  add-todo     Add a new task to your Kanban Board
  add-row      Add a new row to your Kanban Board
  remove-row   Remove an existing Row to your Kanban Board
  remove-task  Remove an existing Task to your Kanban Board
  move-task    Move a task from an origin row to a target row
  open         Open your project with Visual Studio Code (Need the code command in your environment variable Path)
  services     Show all the docker services of your project (docker-compose is required)
  cmd          execute a docker-compose command to your project (docker-compose is required)
  help         Print this message or the help of the given subcommand(s)

Options:
      --kanban <KANBAN>    Show all Todos
  -t, --templates          See all templates available
      --login              Login with Github
      --projects           Show my projects
      --project <PROJECT>  Find my project
      --me                 Show my Github account information
  -h, --help               Print help
  -V, --version            Print version
```
## Environments
```dosini
GITHUB_CLIENT_SECRET=
GITHUB_CLIENT_ID=
GITHUB_REDIRECT_URI=http://127.0.0.1:8100/login
GITHUB_BASE_URL=https://github.com
GITHUB_SCOPE=repo,user
GITHUB_STATE=celerity.io
GITHUB_EXTRA="allow_signup=true"
GITHUB_API_BASE_URL=https://api.github.com

# Note: See more about template file in examples/templates
TEMPLATE_FOLDER=<YOUR_TEMPLATES_FILES_FOLDER>

CELERITY_FOLDER=.celerity/
CONFIG_FOLDER=./.config/

CELERITY_FILE=.celerity/project.json
AVATAR_FILE=./.config/avatar.png
CONFIG_FILE=.config/configuration.json
```
## CLI
For help command use
```bash
# Help command
cargo run --bin celerity -- --help
# Show the current version
cargo run --bin celerity -- --version
```
To show all available templates use
```bash
cargo run --bin celerity -- --templates
```
To login with github
```bash
cargo run --bin celerity -- --login
```
For initialize a new project use
```bash
cargo run --bin celerity -- init --template \
<YOUR_TEMPLATE> \
--name <YOUR_PROJECT_NAME> \
--path <YOUR_PROJECT_PATH>
```
## Desktop App
Make sure that you've install the tauri command before running the project in dev mode  

`Important`: You need to create a `.env.desktop` file for starting the desktop app.  
See the example file on the root of the project.
```bash
cargo tauri dev
```
## Notes
You can add more templates by adding files templates to
`examples/templates` in the repository directory if you want to have more template for testing the tools  
this is only the development public repository but a beta version will coming soon.  

# Todo
- [x] Initialize a project with differents options
    - [x] Choose a project template
    - [x] Create a Github repository and push it in Github
        - [x] Integrate github Authentication with Token
        - [x] Refresh the token when it's expired
    - [x] Save all configurations in json project file
    - [x] Allow the user to create his own template by using a json file description
    - [x] If there's a model then allow to take the figma link of the project
        - [x] Can give a name to the template
        - [x] Specify where can it fetches the template base project
            - On Github Repository (Only github for the first version but all the rests will coming soon)
- [ ] Basics projects template
    - [x] Node.js (Javascript / Typescript)
    - [ ] Nest.js
    - [ ] Angular
    - [ ] React
    - [ ] Vue.js
    - [ ] Svelt
    - [ ] Solid
    - [ ] C++
    - [ ] Rust
    - [ ] Tauri
        - [ ] React (Javascript / Typescript)
        - [ ] Angular
        - [ ] Vue.js
    - [ ] Next.js (Javascript / Typescript)
    - [ ] Nuxt.js (Javascript / Typescript)
- [x] Show the list of your project templates
    - Show the name of the template
    - Show the Author of the template
- [x] Show autheticated user information from github
- [x] Have a todolist for all projects
    - [x] Generate an empty Todolist for the project
    - [x] Show the state and progression of the project based to the Todolist
- [x] Create a CLI tool
    - [x] Command that allow you to connect to your github account
    - [x] Command that show all your templates
    - [x] Command that create a new project from an existing template
    - [x] Command that show the user information from github
    - [x] Command that show all the project and folders
    - [x] Command that open a specific project in vscode
- [ ] Create a GUI
    - [x] Create a GUI model
    - [ ] Allow you to see the list of all project with:
        - Creation Date
        - Last commit and the current branch of the project
        - Name of the project and repository
        - Button: Start the project with Docker
            - If Docker is not found then open the default browser to Docker website
        - Button: Open the project with VS Code
            - If VS Code is not found then open the default browser to VS Code website
        - Button: Browse File
        - Short image or Logo of the project (if None then show the default image)
