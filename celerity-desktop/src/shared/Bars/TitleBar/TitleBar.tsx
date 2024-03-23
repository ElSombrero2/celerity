import './TitleBar.scss'
import { TitleInput } from './components/TitleInput/TitleInput'
import { Buttons } from './components/Buttons/Buttons'

export const TitleBar = () => {

    return (
        <header data-tauri-drag-region className="title-nav top-0 left-0 fixed flex items-center justify-between border-b pl-3">
            <div>
                <img src="/logo-text.svg" className="relative h-8" />
            </div>
            <TitleInput />
            <Buttons />
        </header>
    )
}