import { User } from "./user"

export interface ConfigurationProject{
    id: string,
    path: string,
    name: string
}

export interface Configuration {
    github_token: string,
    user: User
    projects: ConfigurationProject[],
}