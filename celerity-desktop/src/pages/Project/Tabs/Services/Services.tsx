import { Button } from '@/ui/components/ui/button'
import { useServices } from '../../../../app/core/hooks/services'
import './Service.scss'
import { Checkbox } from '@/ui/components/ui/checkbox'

export const Services = ({project}: {project?: string}) =>  {

    const { services } = useServices(project)

    return (
        <div className="services">
            <table>
                <thead>
                    <tr className="table-row">
                        <th><Checkbox /></th>
                        <th>Name</th>
                        <th>Image</th>
                        <th>State</th>
                        <th>Ports</th>
                        <th>Size</th>
                        <th>Project</th>
                        <th>Action</th>
                    </tr>
                </thead>
                <tbody>
                    {services.map((service, index) => (
                        <tr key={`${service.name}-${index}`} className="table-row">
                            <td><Checkbox /></td>
                            <td>{service.name}</td>
                            <td>{service.image}</td>
                            <td>{service.state}</td>
                            <td>{service.ports?.length ? service.ports : '-'}</td>
                            <td>{service.size}</td>
                            <td>{service.project}</td>
                            <td>
                                <Button variant="outline">
                                    <span className="fa fa-play"></span>
                                </Button>
                            </td>
                        </tr>
                    ))}
                </tbody>
            </table>
        </div>
    )
}