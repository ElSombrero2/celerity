import { Link } from 'react-router-dom'
import { useAppSelector } from '../../app/store'
import './Home.scss'

export const Home = () => {

    const config = useAppSelector(state => state.ConfigurationReducer.configuration)

    return (
        <div className="home-container">
            <h1>Hello, Elsombrero</h1>
            <ul>
                {config?.projects.map((p) => (
                    <li key={p.id}>
                        <Link to={`project/${p.id}`}>
                            {p.name}
                        </Link>
                    </li>
                ))}
            </ul>
        </div>
    )
}