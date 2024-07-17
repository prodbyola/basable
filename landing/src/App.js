import React from 'react';
import Header from './components/Header';
import Hero from './components/Hero';
import './App.css';
import Benefits from './components/benefits/Benefits';


const App = () => {
  return (
    <div className="App">
      <Header />
      <Hero />
      <Benefits/>
      
    </div>
  );
};

export default App;
