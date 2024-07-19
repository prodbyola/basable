import React from 'react';
import Header from './components/Header';
import Hero from './components/Hero';
import Benefit from './components/Benefit';
import Features from './components/Features/Features';
import Footer from './components/Footer/Footer';
import Contact from './components/Contact/Contact';

import './App.css';


const App = () => {
  return (
    <div className="App">
      <Header />
      <Hero />
      <Features />    
      <Benefit />
      <Contact />
      <Footer  />
    </div>
  );
}

export default App;
