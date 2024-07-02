import React from 'react';
import logo from '../images/logo.svg'
import linkedin from "../images/LinkedIn - Original.svg";
import tweeter from "../images/prime_twitter.svg";
import github from "../images/Github - Original.svg";

function Footer() {
  const currentYear = new Date().getFullYear();


  return (
    <div className="Footer">
      <div className="footer-container">
        <div className="footer-left">
          <img className="footer-logo" src={logo} alt="logo" />
          <p>
            Open-source data management, visualization, and business insights
            for everyone.
          </p>
        </div>

        <div className="footer-right">
          <div className="footer-right-sections">
            <h4 className="footer-link-header-text">Our Services</h4>
            <div className="footer-right-link">
              <a href="#">Help Center</a>
              <a href="#">FAQ</a>
              <a href="#">Email</a>
            </div>
          </div>

          <div className="footer-right-sections">
            <h4 className="footer-link-header-text">Company</h4>
            <div className="footer-right-link">
              <a href="#">About us</a>
              <a href="#">Career</a>
              <a href="#">Management</a>
              <a href="#">Blog</a>
            </div>
          </div>

          <div className="footer-right-sections">
            <h4 className="footer-link-header-text">Follow Us</h4>
            <div className="footer-right-link footer-right-link-small-screen">
              <a href="#">
                <img className="footer-socials" src={linkedin} alt="linkedin" />
              </a>
              <a href="#">
                <img src={tweeter} alt="twitter" />
              </a>
              <a href="#">
                <img src={github} alt="github" />
              </a>
            </div>
          </div>
        </div>
      </div>
      <div className="footer-bottom">
        <p className='copyright-container'>
          <span className='copyright'>&copy;</span> {currentYear} BASABLE
        </p>
        <p>All Rights Reserved</p>
      </div>
    </div>
  );
}

export default Footer
