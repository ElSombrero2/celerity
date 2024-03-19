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
                <img src={avatar} className="avatar" />
                <Link to="">
                    <i className="fa-solid fa-home"></i>
                </Link>
            </div>
            <div className="items">
                <i className="fa-solid fa-gear"></i>
            </div>
        </div>
    )
}