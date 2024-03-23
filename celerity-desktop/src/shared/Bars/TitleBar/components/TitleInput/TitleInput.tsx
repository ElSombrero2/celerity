import './TitleInput.scss'

export const TitleInput = () => {
    return (
        <div className="input">
            <input type="text" />
            <span className="search-decoration absolute pointer-events-none text-sm">
                <i className="fa-solid fa-magnifying-glass"></i>
                &nbsp;
                Celerity.io
            </span>
        </div>
    )
}