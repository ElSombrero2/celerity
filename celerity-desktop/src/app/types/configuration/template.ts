export interface TemplatePath {
    source: string,
    uri: string,
    branch?: string
}

export interface Template {
    name: string,
    author: string,
    path: TemplatePath,
    description: string
}