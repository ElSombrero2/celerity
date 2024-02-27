# ✨ Celerity.io
Is a simple project manager that include a CLI tool help you to manage your projects and generate them based on a simple template.  
## Why should I use Celerity.io? 🤔
Because it's "Vita Malagasy". 🫡
# How to run
Considere that this tool is `only available in dev mode` now  
It will allow you to init a new project from the existing templates,  
and will show all the existing template in the example templates folder  

`Celerity.io` will search your config to .config/configuration.json  
If the file does not exist then run the Github Login command.  
Running this command will initialize all the use configuration.  
For more information refer to the help section or write an issue on this repository.
# Help

```bash
Celerity is a simple tool to init your project based on templates

Usage: celerity.exe [OPTIONS] [COMMAND]

Commands:
  init
  help  Print this message or the help of the given subcommand(s)

Options:
  -t, --templates     See all templates available
  -g, --github-login  Login with Github
  -h, --help          Print help
  -V, --version       Print version
```
## Environments
```dosini
CLIENT_SECRET=<YOUR GITHUB APP CLIENT SECRET>
CLIENT_ID=<YOUR GITHUB APP CLIENT ID>
REDIRECT_URI=<YOUR GITHUB APP REDIRECT URI>
GITHUB_BASE_URL=https://github.com
SCOPE=<YOUR SCOPE>
STATE=celerity.io
EXTRA="allow_signup=true"
GITHUB_API_BASE_URL=https://api.github.com
```
## Commands 
For help command use
```bash
# Help command
cargo run -- --help
# Show the current version
cargo run -- --version
```
To show all available templates use
```bash
cargo run -- --templates
```
To login with github
```bash
cargo run -- --github-login
# Or
cargo run -- -g

```
For initialize a new project use
```bash
cargo run -- init --template \
<YOUR_TEMPLATE> \
<YOUR_PROJECT_NAME> \
<YOUR_PROJECT_PATH>
```
## Notes
You can add more templates by adding files templates to
`examples/templates` in the repository directory if you want to have more template for testing the tools  
this is only the development public repository but a beta version will coming soon.  

# Todo
- [ ] Initialize a project with differents options
    - [x] Choose a project template
    - [x] Create a Github repository and push it in Github
        - [x] Integrate github Authentication with Token
        - [x] Refresh the token when it's expired
    - [x] Save all configurations in json project file
    - [x] Allow the user to create his own template by using a json file description
    - [ ] Initialize all files and push them to the user's Github Repository
    - [ ] If there's a model then allow to take the figma link of the project
        - [x] Can give a name to the template
        - [x] Specify where can it fetches the template base project
            - On Github Repository (Only github for the first version but all the rests will coming soon)
    - [ ] Basics projects template
        - [ ] Node.js (Javascript / Typescript)
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
        - [ ] Next.js
- [x] Show the list of your project templates
    - Show the name of the template
    - Show the Author of the template
- [ ] Show the list of all current project in your based project folder
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
- [ ] Have a todolist for all projects
    - [ ] Generate an empty Todolist for the project
    - [ ] Show the state and progression of the project based to the Todolist
- [ ] Create a CLI tool
    - [x] Command that allow you to connect to your github account
    - [x] Command that show all your templates
    - [x] Command that create a new project from an existing template
    - [ ] Command that show all the project and folder
    - [ ] Command that allow you to add an existing folder to the project
    - [ ] Command that open a specific project in vscode
- [ ] Create a GUI
    - [ ] Create a GUI model
