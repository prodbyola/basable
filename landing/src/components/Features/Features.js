import React from 'react'
import Heading from './Heading';
import MainSection from './MainSection';
import Explore from './Explore';
import './Features.css'

function Features() {
  return (
    <div className='mainFeatures'>
        <Heading />
        <MainSection />
        <Explore />
      
    </div>
  )
}

export default Features;