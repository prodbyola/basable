import React from 'react';
import './Header.scss';
import Logo from './assets/logo.svg';
import LinkedIn from './assets/LinkedIn.svg';
import X from './assets/X.svg';
import Github from './assets/Github.svg';

const Header = () => {
  return (
    <header className="header">
      <div className="container">
        <img src={Logo} alt="Logo" className="logo" />
        <nav className="nav">
          <a href="#about" className="nav-link">About us</a>
          <a href="#features" className="nav-link">Features</a>
          <a href="#contact" className="nav-link">Contact us</a>
        </nav>
        <div className="header-buttons">
          <button className="btn login">Join now</button>
          <a href="https://www.linkedin.com" target="_blank" rel="noopener noreferrer">
            <img src={LinkedIn} alt="LinkedIn" className="icon" />
          </a>
          <a href="https://www.twitter.com" target="_blank" rel="noopener noreferrer">
            <img src={X} alt="X" className="icon" />
          </a>
          <a href="https://www.github.com" target="_blank" rel="noopener noreferrer">
            <img src={Github} alt="GitHub" className="icon" />
          </a>
        </div>
      </div>
    </header>
  );
};

export default Header;
