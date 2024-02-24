# Celerity.io
Is a simple project manager that include a CLI tool help you to manage your projects and generate them based on a simple template.  
It will group them in a single folder and you can easily search a project, open it or remove it by a simple command.  

# Todo
- [ ] Initialize a project with differents options
    - [x] Choose a project template
    - [ ] Create a Github repository and push it in Github
        - [ ] Integrate github Authentication with Token
    - [ ] Initialize all files and push them to the user's Github Repository
    - [ ] If there's a model then allow to take the figma link of the project
    - [x] Save all configurations in json project file
    - [x] Allow the user to create his own template by using a json file description
        - [x] Can give a name to the template
        - [x] Specify where can he found the templates files
            - On Github Repository
            - In the local file
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
- [ ] Show the list of all current projects
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
- [ ] Create a GUI
    - [ ] Create a GUI model