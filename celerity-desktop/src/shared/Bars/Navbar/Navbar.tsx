import { useEffect, useState } from 'react'
import './Navbar.scss'
import { invoke } from '@tauri-apps/api';
import { Link } from 'react-router-dom';

export const Navbar = () => {

    const [avatar, setAvatar] = useState('')

    useEffect(() => {
        invoke('get_configuration').then((data: any) => setAvatar(data.user.avatar_url))
    }, []);
    
    return (
        <div className="navbar">
            <div className="items">
                <Link to="">
                    <i className="fa-solid fa-list"></i>
                </Link>
                <Link to="project/4585">
                    <i className="fa-solid fa-diagram-project"></i>
                </Link>
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