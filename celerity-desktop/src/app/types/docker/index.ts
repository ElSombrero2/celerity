
export interface DockerServices {
    id?: string,
    name: string,
    image?: string,
    ports?: string,
    project: string,
    state: string,
    size: string,
}