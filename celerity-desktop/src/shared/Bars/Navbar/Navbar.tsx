import { useEffect, useState } from 'react'
import './Navbar.scss'
import { invoke } from '@tauri-apps/api';
import { Link } from 'react-router-dom';
import { useMaximized } from '@/app/core/hooks/maximized/maximized';

export const Navbar = () => {

    const [avatar, setAvatar] = useState('')
    const maximized = useMaximized();

    useEffect(() => {
        invoke('get_configuration').then((data: any) => setAvatar(data.user.avatar_url))
    }, []);
    
    return (
        <div className={`navbar flex flex-col gap-5 items-center bg-slate-900 justify-between left-0 fixed ${maximized && 'maximized'}`}>
            <div className="flex flex-col gap-3 items-center">
                <img src={avatar} className="w-6 h-6 object-contains object-center rounded-full" />
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