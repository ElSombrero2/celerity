import { useState } from "react"

export const useLoading = (initial: boolean) => {
    const [loading, setLoading] = useState(initial)

    const start = () => setLoading(true)  
    const stop = () => setLoading(false)
    
    return { loading, start, stop }
}