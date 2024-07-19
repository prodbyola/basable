import React from 'react';
import Header from './components/Header';
import Hero from './components/Hero';
import Benefit from './components/Benefit';
import Features from './components/Features/Features';
import Footer from './components/footer';
import Contact from './components/contact';
import { Analytics } from "@vercel/analytics/react"

import './App.css';


const App = () => {
  return (
    <div className="App">
      <Header />
      <Hero />
      <Features />    
      <Benefit />
      <Contact />
      <Footer />
      <Analytics />
    </div>
  );
}

export default App;
