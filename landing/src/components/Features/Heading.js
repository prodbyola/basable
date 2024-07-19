import React from 'react'
import logo from './Logo.svg'
import './Heading.css'


function Heading() {
  return (
    <div className='mainHeading'>


        <div className='head'>
          <img src={logo} className='logo' alt= 'Basable Logo' width='50px' height='50px'/>
          <span id='features'>Features</span>
        </div>
        <div className='openSource'>
          <h1 id='source'>Your <span id='open'>Open-Source</span> Data <span id='power'>Powerhouse</span> </h1>
        </div>



    </div>
  )
}






export default Heading;