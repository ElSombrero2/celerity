export interface User{
    login: string,
    id: number,
    avatar_url: string,
    gravatar_id: string,
    url: string,
    name: string,
    email: string,
    bio: string,
    public_repos: number,
    public_gists: number,
    followers: number,
    following: number,
    created_at: string,
    updated_at: string,
    private_gists: number,
    total_private_repos: number,
    owned_private_repos: number
}