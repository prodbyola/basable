import React from 'react'
import './Features.css'
import LogoAndFeatures from './LogoAndFeatures2.svg'
import openSource from './Image2.svg'
import Effortless from './Effortless3.svg'
import Stunning from './Stunning3.svg'
import Action from './Actionable3.svg'
import Enhanced from './Enhanced3.svg'
import explore from './Explore1.svg'
import mobileEffort from './EffortForMobile1.svg'
import mobileAction from './ActionForMobile1.svg'




function Features() {
  return (
    <div className='featureSection'>

        <div className='Section'>
      
         <div className='mainFeatures' >
                    <div className='features'>
                            <img src= {LogoAndFeatures} className='heading' alt='Features' width='260px' height='66px' />      
                    </div>
                   <div className='openSource'>
                           <img src={openSource} className='mainHeading' alt='Your Open Source Data' width='700px' height='150px' />
                   </div>
          </div>


         <div className='main'>

                 <div className='section1'>
                        <img src={Effortless} className='effort' alt='Effortless Data Management' width='824px' height='690.21px' />
                        
                        <img src={mobileEffort}  className='mobileEffort' alt='Effortless Data' />
                        
                 </div>

                <div className='section2'>
                       <img src={Stunning} className='stunning' alt='Stunning Data Visualization' width='400px' height='334px' />
                
                </div>

                <div className='section3'>
                       <img src={Action} className='action' alt='Actionable Business Insights' width='1248px' height='449px'/>
                     <img src={mobileAction} className='mobileAction' alt='Actionable Businness' />
                      
                </div>
            
               <div className='section4'>
                       <img src={Enhanced} className='enhanced' alt='Enhanced Collaboration' width='400px' height='334px' />
                
               </div>

         </div>

        <div className='exploreMore'>
             <a href='/' >
                  <img src={explore} className='explore' alt='Explore More' width='178px' height='56px' />
              </a>
         </div>

     </div>



    </div>
  )
}






export default Features;