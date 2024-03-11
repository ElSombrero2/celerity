import './TitleBar.scss'
import { TitleInput } from './components/TitleInput/TitleInput'
import { Buttons } from './components/Buttons/Buttons'


export const TitleBar = () => {

    return (
        <header data-tauri-drag-region className="title-nav">
            <div>
                <img style={{position: 'relative', top:'3px'}} src="/logo-text.svg" height={35} />
            </div>
            <TitleInput />
            <Buttons />
        </header>
    )
}