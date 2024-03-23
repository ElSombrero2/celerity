import { Link } from 'react-router-dom'
import { useAppSelector } from '../../app/store'
import './Home.scss'
import { Card } from '@/ui/components/ui/card'

export const Home = () => {

    const config = useAppSelector(state => state.ConfigurationReducer.configuration)

    return (
        <div className="home-container">
            <h1>Hello, </h1>
            <Card>
                <h3>News of the day</h3>
                
            </Card>
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