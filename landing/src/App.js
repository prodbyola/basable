import React from 'react';
import Header from './components/Header';
import Hero from './components/Hero';
import './App.css';
import Features from './components/Features_Section/Features';


const App = () => {
  return (
    <div className="App">
      <Header />
      <Hero />
      <Features />
      
    </div>
  );
}

export default App;
