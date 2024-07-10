import React from 'react';
import './Hero.scss';
// import backgroundPattern from './assets/Background-pattern.svg';
// import dashboard from './assets/dashboard.svg';
// import table from './assets/table.svg';
// import profileBar from './assets/profile-bar.svg';
// import vUp from './assets/v-up.svg';
// import vDown from './assets/v-down.svg';
// import colorBar from './assets/Vector.svg';

const getImage = (name) => '/images/hero/'+name+'.svg'

const Hero = () => {
  return (
    <section id="hero" className="hero">
      <div className="grid-pattern" style={{ backgroundImage: `url(${getImage('Background-pattern')})` }}></div>
      <img src={getImage('Vector')} alt="Color Bar" className="color-bar-image" />
      <div className="hero-content">
        <h1>
          <span className="highlight">Unleash</span> <span>the Power <br/> of Your Data</span>
        </h1>
        <p>Open-source data management, visualization, and business insights for everyone.</p>
        <button className="btn primary-btn">Join Waiting List</button>
      </div>
      <div className="hero-image">
        <img src={getImage('dashboard')} alt="Dashboard" className="dashboard-image" />
        <img src={getImage('table')} alt="Table" className="table-image" />
        <img src={getImage('profile-bar')} alt="Profile Bar" className="profile-bar-image" />
        <img src={getImage('v-down')} alt="Visualize Bar Down" className="v-down-image" />
        <img src={getImage('v-up')} alt="Visualize Bar Up" className="v-up-image" />
      </div>
    </section>
  );
};

export default Hero;
