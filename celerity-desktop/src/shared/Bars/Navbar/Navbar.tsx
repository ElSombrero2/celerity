import { useEffect, useState } from 'react'
import './Navbar.scss'
import { invoke } from '@tauri-apps/api';

export const Navbar = () => {

    const [avatar, setAvatar] = useState('')

    useEffect(() => {
        invoke('get_configuration').then((data: any) => setAvatar(data.user.avatar_url))
    }, []);
    
    return (
        <div className="navbar">
            <div className="items">
                <i className="fa fa-plus"></i>
                <i className="fa-solid fa-list"></i>
                <i className="fa-solid fa-diagram-project"></i>
                <i className="fa-solid fa-bars-progress"></i>
                <i className="fa-solid fa-chart-simple"></i>
            </div>
            <div className="items">
                <img src={avatar} className="avatar" />
                <i className="fa-solid fa-gear"></i>
            </div>
        </div>
    )
}