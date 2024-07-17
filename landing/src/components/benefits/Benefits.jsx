import styled from 'styled-components'
import './Benefits.scss'
import OPENSOURCE from './assets/open-source.svg'
import COSTEFFECTIVE from './assets/cost-effectiveness.svg'
import TEAM from './assets/team.svg'
import BLOGO from './assets/logo-b.svg'

const Wrapper = styled.section`
    margin-top: 140px;
`

const Benefits = () => {


    return ( 
        <Wrapper>
            <div >
                <div className='btn-benefit'>
                <span>
                    <figure><img src={BLOGO} alt="benefits" /></figure>
                </span>
                <span>Benefits</span>
                </div>
            </div>
            <h1>Why Our Product is Different
            </h1>

        {/* card one */}
        <main className='card-container-one'>
            
            <div className='card'>
            <figure>
                <img src={OPENSOURCE} alt="opensource" />
                </figure>
            <h3>Open Source Freedom</h3>
            <p>Empower your team with complete control and customization. No vendor lock-in</p>

            </div>
        </main>


        <main className='card-container-two'>

        {/* card two */}
        <div className='card'>
            <figure>
                <img src={COSTEFFECTIVE} alt="opensource" />
                </figure>
            <h3>Cost Effective Solution</h3>
            <p>Leverage the power of open-source and avoid expensive commercial licenses</p>

            </div>

        {/* card three */}
        <div className='card'>
            <figure>
                <img src={TEAM} alt="collaboration insights" />
                </figure>
            <h3>Collaboartion Insights</h3>
            <p>Share dashboards and facilitate data-driven decison making</p>

            </div>
        </main>

          </Wrapper>
     );
}
 
export default Benefits;